use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipUser {
    pub user_id: i64,
    pub full_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub avatar_version: Option<i64>,
    pub is_active: bool,
    pub is_bot: bool,
    pub role: i64,
    pub timezone: Option<String>,
    pub date_joined: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipStream {
    pub stream_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub rendered_description: Option<String>,
    pub invite_only: bool,
    pub is_announcement_only: bool,
    pub is_web_public: bool,
    pub history_public_to_subscribers: bool,
    pub first_message_id: Option<i64>,
    pub stream_post_policy: Option<i64>,
    pub message_retention_days: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipMessage {
    pub id: i64,
    pub sender_id: i64,
    pub sender_full_name: String,
    pub sender_email: String,
    pub sender_realm_str: Option<String>,
    pub content: String,
    pub rendered_content: Option<String>,
    pub content_type: String,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub stream_id: Option<i64>,
    pub subject: Option<String>,
    pub subject_links: Option<Vec<String>>,
    pub display_recipient: Option<serde_json::Value>,
    pub reactions: Option<Vec<ZulipReaction>>,
    pub flags: Option<Vec<String>>,
    pub last_edit_timestamp: Option<i64>,
    pub edit_history: Option<Vec<serde_json::Value>>,
}

impl ZulipMessage {
    pub fn is_stream(&self) -> bool {
        self.msg_type == "stream"
    }

    pub fn is_private(&self) -> bool {
        self.msg_type == "private"
    }

    pub fn topic(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    pub fn recipient_user_ids(&self) -> Vec<i64> {
        if let Some(recipients) = &self.display_recipient {
            if let Some(arr) = recipients.as_array() {
                return arr.iter().filter_map(|r| r.get("id")?.as_i64()).collect();
            }
        }
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipReaction {
    pub emoji_name: String,
    pub emoji_code: String,
    pub reaction_type: String,
    pub user_id: i64,
    pub user: Option<ZulipUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub id: Option<i64>,
    pub message: Option<ZulipMessage>,
    pub user_id: Option<i64>,
    pub stream_id: Option<i64>,
    pub stream_name: Option<String>,
    pub message_id: Option<i64>,
    pub reaction: Option<ZulipReaction>,
    pub emoji_name: Option<String>,
    pub emoji_code: Option<String>,
    pub op: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl ZulipEvent {
    pub fn is_message(&self) -> bool {
        self.event_type == "message"
    }

    pub fn is_reaction(&self) -> bool {
        self.event_type == "reaction"
    }

    pub fn is_update_message(&self) -> bool {
        self.event_type == "update_message"
    }

    pub fn is_delete_message(&self) -> bool {
        self.event_type == "delete_message"
    }

    pub fn is_subscription(&self) -> bool {
        self.event_type == "subscription"
    }

    pub fn is_realm_user(&self) -> bool {
        self.event_type == "realm_user"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipQueue {
    pub queue_id: String,
    pub last_event_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipApiResponse<T> {
    pub result: String,
    pub msg: String,
    #[serde(flatten)]
    pub data: Option<T>,
}

impl<T> ZulipApiResponse<T> {
    pub fn is_success(&self) -> bool {
        self.result == "success"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipMessagesResponse {
    pub messages: Vec<ZulipMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipStreamsResponse {
    pub streams: Vec<ZulipStream>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipUsersResponse {
    pub members: Vec<ZulipUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipSendMessageResponse {
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZulipEventsResponse {
    pub events: Vec<ZulipEvent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SendMessageRequest {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
}

impl SendMessageRequest {
    pub fn stream(stream_id: i64, topic: &str, content: &str) -> Self {
        Self {
            msg_type: "stream".to_string(),
            to: None,
            stream_id: Some(stream_id),
            topic: Some(topic.to_string()),
            content: content.to_string(),
            local_id: None,
            queue_id: None,
        }
    }

    pub fn private(user_ids: &[i64], content: &str) -> Self {
        Self {
            msg_type: "private".to_string(),
            to: Some(
                user_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            stream_id: None,
            topic: None,
            content: content.to_string(),
            local_id: None,
            queue_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterQueueRequest {
    pub event_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_public_streams: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subscribers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_gravatar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slim_presence: Option<bool>,
}

impl Default for RegisterQueueRequest {
    fn default() -> Self {
        Self {
            event_types: vec![
                "message".to_string(),
                "reaction".to_string(),
                "update_message".to_string(),
                "delete_message".to_string(),
                "subscription".to_string(),
                "realm_user".to_string(),
            ],
            all_public_streams: Some(true),
            include_subscribers: Some(false),
            client_gravatar: Some(true),
            slim_presence: Some(true),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageFlag {
    Read,
    Starred,
    Collapsed,
    Mentioned,
    WildcardMentioned,
    HasAlertWord,
    Historical,
}

impl MessageFlag {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageFlag::Read => "read",
            MessageFlag::Starred => "starred",
            MessageFlag::Collapsed => "collapsed",
            MessageFlag::Mentioned => "mentioned",
            MessageFlag::WildcardMentioned => "wildcard_mentioned",
            MessageFlag::HasAlertWord => "has_alert_word",
            MessageFlag::Historical => "historical",
        }
    }
}
