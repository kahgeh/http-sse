use actix_web::{get, post, Responder, HttpResponse, web::{Data}, HttpRequest};
use tracing::{error,info};
use crate::sse_exchange::{SseExchange, Event};
use actix_web::web::Bytes;
use crate::settings::Settings;
use crate::peers::{Compute, PeerEndpoint};
use crate::contracts::{EventBroadcastRequest};
use actix_web::web::Json;

#[get("/clients/{client_id}/events")]
pub async fn receive_connect_request(req: HttpRequest, sse_exchange: Data<SseExchange>)-> impl Responder {
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

#[post("/clients/{client_id}/events")]
pub async fn receive_send_request(req: HttpRequest, body: Bytes,
                                  settings: Data<Settings>,
                                  discovery_service: Data<Compute>,
                                  sse_exchange: Data<SseExchange>) -> impl Responder {
    let client_id = req.match_info().query("client_id");

    let result_converting_body_to_string = String::from_utf8(body.to_vec());

    if result_converting_body_to_string.is_err() {
        error!("there is an issue with the payload");
        return HttpResponse::BadRequest()
            .finish();
    }

    let payload = result_converting_body_to_string.unwrap();

    if settings.broadcast_event {
        info!("broadcast to peers");
        return match discovery_service.get_service_endpoints().await {
            Ok(peer_endpoints) => {
                for peer_endpoint in peer_endpoints {
                    let PeerEndpoint{ip, port} = peer_endpoint;
                    info!(ip=&ip[..],"broadcasting");
                    let client = reqwest::Client::new();
                    client.post(format!("http://{}:{}/broadcasts/events", &ip, port))
                        .header("client-id", client_id)
                        .json(&EventBroadcastRequest{
                            client_id: String::from(client_id),
                            payload: payload.clone(),
                        })
                        .send()
                        .await.expect("fail to broadcast");
                }
                HttpResponse::Ok().finish()
            },
            Err(_) => {
                error!("fail to determine peers");
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    if !sse_exchange.publish(Event::new(client_id, &payload[..])).await {
        error!("fail to send events");
        return HttpResponse::InternalServerError()
            .finish();
    }
    HttpResponse::Ok().finish()
}

#[post("broadcasts/events")]
pub async fn receive_send_broadcast(broadcast_request: Json<EventBroadcastRequest>, sse_exchange: Data<SseExchange>) -> impl Responder{
    if !sse_exchange.publish(broadcast_request.into()).await {
        error!("fail to send events");
        return HttpResponse::InternalServerError()
            .finish();
    }
    HttpResponse::Ok().finish()
}