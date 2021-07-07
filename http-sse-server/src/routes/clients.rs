use actix_web::{get, put, Responder, HttpResponse, web::{Data}, HttpRequest};
use tracing::{error};
use crate::sse_exchange::{SseExchange, Event};
use actix_web::web::Bytes;

#[get("/clients/{client_id}/events")]
pub async fn subscribe(req: HttpRequest, sse_exchange: Data<SseExchange>)-> impl Responder {
    let client_id = req.match_info().query("client_id");
    match (*sse_exchange).connect(client_id).await {
        Ok(rx)=>{
            HttpResponse::Ok()
                .append_header(("content-type", "text/event-stream"))
                .append_header(("cache-control", "no-cache"))
                .append_header(("connection", "keep-alive"))
                .append_header(("access-control-allow-origin", "*"))
                .streaming(rx)
        },
        Err(_)=> {
            error!("fail to establish connection");
            HttpResponse::InternalServerError()
                .finish()
        }
    }
}

#[put("/clients/{client_id}/events")]
pub async fn publish(req: HttpRequest, body: Bytes, sse_exchange: Data<SseExchange>) -> impl Responder {
    let client_id = req.match_info().query("client_id");

    let result_converting_body_to_string = String::from_utf8(body.to_vec());

    if result_converting_body_to_string.is_err() {
        error!("there is an issue with the payload");
        return HttpResponse::BadRequest()
            .finish();
    }

    let payload = result_converting_body_to_string.unwrap();

    if !(*sse_exchange).publish(Event::new(client_id, &payload[..])).await {
        error!("fail to send events");
        return HttpResponse::InternalServerError()
            .finish();
    }
    HttpResponse::Ok().finish()
}