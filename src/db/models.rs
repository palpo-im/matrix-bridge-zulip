use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMapping {
    pub id: i64,
    pub matrix_room_id: String,
    pub zulip_stream_id: i64,
    pub zulip_stream_name: String,
    pub organization_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMapping {
    pub id: i64,
    pub matrix_event_id: String,
    pub matrix_room_id: String,
    pub zulip_message_id: i64,
    pub zulip_sender_id: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMapping {
    pub id: i64,
    pub matrix_user_id: String,
    pub zulip_user_id: i64,
    pub zulip_email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationData {
    pub name: String,
    pub site: Option<String>,
    pub email: Option<String>,
    pub api_key: Option<String>,
    pub connected: bool,
    pub max_backfill_amount: i32,
}
