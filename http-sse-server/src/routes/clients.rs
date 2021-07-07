use actix_web::{get, Responder, HttpResponse, web::{Data}, HttpRequest};
use tracing::{error};
use crate::sse_exchange::{SseExchange};

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