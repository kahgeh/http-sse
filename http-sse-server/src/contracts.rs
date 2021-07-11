use serde::{Deserialize,Serialize};

#[derive(Serialize)]
pub struct GetAppInfoResponse {
    pub app_name: String,
    pub git_commit_id: String,
    pub started : String,
    pub current_time : String,
}

#[derive(Serialize, Deserialize)]
pub struct EventBroadcastRequest {
    pub client_id: String,
    pub payload: String,
}