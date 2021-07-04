use actix_web::{get, Responder,Error, HttpResponse, web};
use futures::{future::ok, stream::once};

#[get("/clients/{client_id}/events")]
pub async fn subscribe()-> impl Responder {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"data: from rust connected\n\n")));
    HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .append_header(("cache-control", "no-cache"))
        .append_header(("connection", "keep-alive"))
        .append_header(("access-control-allow-origin", "*"))
        .streaming(body)
}