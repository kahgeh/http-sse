use crate::contracts::EventBroadcastRequest;
use crate::sse_exchange::Event;
use actix_web::web::Json;

impl From<Json<EventBroadcastRequest>> for Event {
    fn from(json: Json<EventBroadcastRequest>) -> Self {
        Event::new(&json.client_id, &json.payload)
    }
}