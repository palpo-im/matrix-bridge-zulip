use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipUser {
    pub user_id: i64,
    pub full_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub role: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipStream {
    pub stream_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub invite_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipMessage {
    pub id: i64,
    pub sender_id: i64,
    pub sender_full_name: String,
    pub content: String,
    pub content_type: String,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub stream_id: Option<i64>,
    pub subject: Option<String>,
    pub display_recipient: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub message: Option<ZulipMessage>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
