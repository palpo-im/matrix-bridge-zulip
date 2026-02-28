use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::organizations)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub site: String,
    pub email: String,
    pub api_key: String,
    pub connected: bool,
    pub max_backfill_amount: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::organizations)]
pub struct OrganizationChangeset {
    pub name: String,
    pub site: String,
    pub email: String,
    pub api_key: String,
    pub connected: bool,
    pub max_backfill_amount: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::room_mappings)]
pub struct RoomMapping {
    pub id: i64,
    pub matrix_room_id: String,
    pub zulip_stream_id: i64,
    pub zulip_stream_name: String,
    pub zulip_topic: Option<String>,
    pub organization_id: String,
    pub room_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::room_mappings)]
pub struct NewRoomMapping {
    pub matrix_room_id: String,
    pub zulip_stream_id: i64,
    pub zulip_stream_name: String,
    pub zulip_topic: Option<String>,
    pub organization_id: String,
    pub room_type: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::user_mappings)]
pub struct UserMapping {
    pub id: i64,
    pub matrix_user_id: String,
    pub zulip_user_id: i64,
    pub zulip_email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_bot: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::user_mappings)]
pub struct NewUserMapping {
    pub matrix_user_id: String,
    pub zulip_user_id: i64,
    pub zulip_email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_bot: bool,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::user_mappings)]
pub struct UserMappingChangeset {
    pub zulip_email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub is_bot: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::message_mappings)]
pub struct MessageMapping {
    pub id: i64,
    pub matrix_event_id: String,
    pub matrix_room_id: String,
    pub zulip_message_id: i64,
    pub zulip_sender_id: i64,
    pub message_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::message_mappings)]
pub struct NewMessageMapping {
    pub matrix_event_id: String,
    pub matrix_room_id: String,
    pub zulip_message_id: i64,
    pub zulip_sender_id: i64,
    pub message_type: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::processed_events)]
pub struct ProcessedEvent {
    pub id: i64,
    pub event_id: String,
    pub event_type: String,
    pub source: String,
    pub processed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::processed_events)]
pub struct NewProcessedEvent {
    pub event_id: String,
    pub event_type: String,
    pub source: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::reaction_mappings)]
pub struct ReactionMapping {
    pub id: i64,
    pub matrix_event_id: String,
    pub zulip_message_id: i64,
    pub zulip_reaction_id: i64,
    pub emoji: String,
    pub matrix_reaction_event_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::reaction_mappings)]
pub struct NewReactionMapping {
    pub matrix_event_id: String,
    pub zulip_message_id: i64,
    pub zulip_reaction_id: i64,
    pub emoji: String,
    pub matrix_reaction_event_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomType {
    Stream,
    Direct,
    Topic,
}

impl RoomType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoomType::Stream => "stream",
            RoomType::Direct => "direct",
            RoomType::Topic => "topic",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "stream" => Some(RoomType::Stream),
            "direct" => Some(RoomType::Direct),
            "topic" => Some(RoomType::Topic),
            _ => None,
        }
    }
}

impl std::fmt::Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Image,
    Video,
    Audio,
    File,
    Emote,
}

impl MessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Text => "text",
            MessageType::Image => "image",
            MessageType::Video => "video",
            MessageType::Audio => "audio",
            MessageType::File => "file",
            MessageType::Emote => "emote",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "text" => Some(MessageType::Text),
            "image" => Some(MessageType::Image),
            "video" => Some(MessageType::Video),
            "audio" => Some(MessageType::Audio),
            "file" => Some(MessageType::File),
            "emote" => Some(MessageType::Emote),
            _ => None,
        }
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
