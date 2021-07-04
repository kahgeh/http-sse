use serde::{Serialize};

#[derive(Serialize)]
pub struct GetAppInfoResponse {
    pub app_name: String,
    pub git_commit_id: String,
    pub started : String,
    pub current_time : String,
}
