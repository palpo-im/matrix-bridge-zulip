pub mod types;
pub mod event_handler;
pub mod websocket;

pub use self::event_handler::ZulipEventHandler;
pub use self::types::{
    RegisterQueueRequest, SendMessageRequest, ZulipApiResponse, ZulipEvent, ZulipEventsResponse,
    ZulipMessage, ZulipMessagesResponse, ZulipQueue, ZulipReaction, ZulipSendMessageResponse,
    ZulipStream, ZulipStreamsResponse, ZulipUser, ZulipUsersResponse,
};
pub use self::websocket::ZulipWebSocketClient;

use reqwest::header::{HeaderMap, AUTHORIZATION};
use tracing::{debug, error, info};
use url::Url;

use crate::utils::{BridgeError, Result};

pub struct ZulipClient {
    site: String,
    email: String,
    api_key: String,
    client: reqwest::Client,
    base_url: Url,
}

impl ZulipClient {
    pub fn new(site: &str, email: &str, api_key: &str) -> Result<Self> {
        let base_url = Url::parse(site)
            .map_err(|e| BridgeError::Zulip(format!("Invalid site URL: {}", e)))?;

        Ok(Self {
            site: site.trim_end_matches('/').to_string(),
            email: email.to_string(),
            api_key: api_key.to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|e| BridgeError::Network(e.to_string()))?,
            base_url,
        })
    }

    fn auth_header(&self) -> String {
        use base64::Engine;
        let credentials = format!("{}:{}", self.email, self.api_key);
        format!("Basic {}", base64::engine::general_purpose::STANDARD.encode(credentials))
    }

    fn api_url(&self, path: &str) -> Result<Url> {
        self.base_url
            .join(&format!("/api/v1/{}", path.trim_start_matches('/')))
            .map_err(|e| BridgeError::Zulip(format!("Invalid API URL: {}", e)))
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.api_url(path)?;
        debug!("GET {}", url);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            self.auth_header().parse().map_err(|e| BridgeError::Zulip(format!("Invalid header: {}", e)))?,
        );

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        debug!("Response status: {}, body: {}", status, body);

        serde_json::from_str(&body).map_err(|e| {
            BridgeError::Zulip(format!("Failed to parse response: {} - {}", e, body))
        })
    }

    async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.api_url(path)?;
        debug!("POST {}", url);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            self.auth_header().parse().map_err(|e| BridgeError::Zulip(format!("Invalid header: {}", e)))?,
        );

        let response = self
            .client
            .post(url)
            .headers(headers)
            .form(body)
            .send()
            .await
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        let status = response.status();
        let response_body = response
            .text()
            .await
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        debug!("Response status: {}, body: {}", status, response_body);

        serde_json::from_str(&response_body).map_err(|e| {
            BridgeError::Zulip(format!("Failed to parse response: {} - {}", e, response_body))
        })
    }

    pub async fn get_profile(&self) -> Result<ZulipUser> {
        let response: ZulipApiResponse<ZulipUser> = self.get("users/me").await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to get profile: {}",
                response.msg
            )));
        }

        response.data.ok_or_else(|| BridgeError::Zulip("No profile data".to_string()))
    }

    pub async fn get_users(&self) -> Result<Vec<ZulipUser>> {
        let response: ZulipApiResponse<ZulipUsersResponse> = self.get("users").await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to get users: {}",
                response.msg
            )));
        }

        Ok(response.data.map(|d| d.members).unwrap_or_default())
    }

    pub async fn get_streams(&self) -> Result<Vec<ZulipStream>> {
        let response: ZulipApiResponse<ZulipStreamsResponse> = self.get("streams").await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to get streams: {}",
                response.msg
            )));
        }

        Ok(response.data.map(|d| d.streams).unwrap_or_default())
    }

    pub async fn get_stream_id(&self, stream_name: &str) -> Result<i64> {
        let response: ZulipApiResponse<serde_json::Value> = self
            .get(&format!("get_stream_id?stream={}", stream_name))
            .await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to get stream ID: {}",
                response.msg
            )));
        }

        response
            .data
            .and_then(|d| d.get("stream_id")?.as_i64())
            .ok_or_else(|| BridgeError::Zulip("No stream_id in response".to_string()))
    }

    pub async fn send_message(&self, request: &SendMessageRequest) -> Result<i64> {
        let response: ZulipApiResponse<ZulipSendMessageResponse> =
            self.post("messages", request).await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to send message: {}",
                response.msg
            )));
        }

        response
            .data
            .map(|d| d.id)
            .ok_or_else(|| BridgeError::Zulip("No message ID in response".to_string()))
    }

    pub async fn send_stream_message(
        &self,
        stream_id: i64,
        topic: &str,
        content: &str,
    ) -> Result<i64> {
        let request = SendMessageRequest::stream(stream_id, topic, content);
        self.send_message(&request).await
    }

    pub async fn send_private_message(&self, user_ids: &[i64], content: &str) -> Result<i64> {
        let request = SendMessageRequest::private(user_ids, content);
        self.send_message(&request).await
    }

    pub async fn get_messages(
        &self,
        stream_id: i64,
        topic: Option<&str>,
        anchor: Option<i64>,
        num_before: i32,
        num_after: i32,
    ) -> Result<Vec<ZulipMessage>> {
        let narrow = if let Some(t) = topic {
            format!(
                r#"[{{"operator":"stream","operand":{}}},{{"operator":"topic","operand":"{}"}}]"#,
                stream_id, t
            )
        } else {
            format!(r#"[{{"operator":"stream","operand":{}}}]"#, stream_id)
        };

        let anchor_str = anchor.map(|a| a.to_string()).unwrap_or_else(|| "newest".to_string());

        let path = format!(
            "messages?narrow={}&anchor={}&num_before={}&num_after={}",
            urlencoding::encode(&narrow),
            anchor_str,
            num_before,
            num_after
        );

        let response: ZulipApiResponse<ZulipMessagesResponse> = self.get(&path).await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to get messages: {}",
                response.msg
            )));
        }

        Ok(response.data.map(|d| d.messages).unwrap_or_default())
    }

    pub async fn get_message(&self, message_id: i64) -> Result<ZulipMessage> {
        let response: ZulipApiResponse<ZulipMessage> =
            self.get(&format!("messages/{}", message_id)).await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to get message: {}",
                response.msg
            )));
        }

        response
            .data
            .ok_or_else(|| BridgeError::Zulip("No message data".to_string()))
    }

    pub async fn edit_message(&self, message_id: i64, content: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct EditMessageRequest {
            content: String,
        }

        let request = EditMessageRequest {
            content: content.to_string(),
        };

        let response: ZulipApiResponse<()> = self
            .post(&format!("messages/{}", message_id), &request)
            .await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to edit message: {}",
                response.msg
            )));
        }

        Ok(())
    }

    pub async fn delete_message(&self, message_id: i64) -> Result<()> {
        #[derive(serde::Serialize)]
        struct DeleteMessageRequest {}

        let request = DeleteMessageRequest {};

        let response: ZulipApiResponse<()> = self
            .post(&format!("messages/{}?allow_deleting_other=1", message_id), &request)
            .await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to delete message: {}",
                response.msg
            )));
        }

        Ok(())
    }

    pub async fn add_reaction(&self, message_id: i64, emoji_name: &str, emoji_code: &str) -> Result<()> {
        #[derive(serde::Serialize)]
        struct AddReactionRequest {
            emoji_name: String,
            emoji_code: String,
            reaction_type: String,
        }

        let request = AddReactionRequest {
            emoji_name: emoji_name.to_string(),
            emoji_code: emoji_code.to_string(),
            reaction_type: "unicode_emoji".to_string(),
        };

        let response: ZulipApiResponse<()> = self
            .post(&format!("messages/{}/reactions", message_id), &request)
            .await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to add reaction: {}",
                response.msg
            )));
        }

        Ok(())
    }

    pub async fn remove_reaction(&self, message_id: i64, emoji_name: &str, emoji_code: &str) -> Result<()> {
        let path = format!(
            "messages/{}/reactions?emoji_name={}&emoji_code={}&reaction_type=unicode_emoji",
            message_id, emoji_name, emoji_code
        );

        let url = self.api_url(&path)?;
        
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            self.auth_header().parse().map_err(|e| BridgeError::Zulip(format!("Invalid header: {}", e)))?,
        );

        let response = self
            .client
            .delete(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        debug!("DELETE reaction response status: {}, body: {}", status, body);

        let api_response: ZulipApiResponse<()> =
            serde_json::from_str(&body).map_err(|e| {
                BridgeError::Zulip(format!("Failed to parse response: {}", e))
            })?;

        if !api_response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to remove reaction: {}",
                api_response.msg
            )));
        }

        Ok(())
    }

    pub async fn register_event_queue(
        &self,
        request: &RegisterQueueRequest,
    ) -> Result<ZulipQueue> {
        let response: ZulipApiResponse<ZulipQueue> = self.post("register", request).await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to register event queue: {}",
                response.msg
            )));
        }

        response
            .data
            .ok_or_else(|| BridgeError::Zulip("No queue data in response".to_string()))
    }

    pub async fn get_events(
        &self,
        queue_id: &str,
        last_event_id: i64,
    ) -> Result<ZulipEventsResponse> {
        let path = format!(
            "events?queue_id={}&last_event_id={}",
            queue_id, last_event_id
        );

        let response: ZulipApiResponse<ZulipEventsResponse> = self.get(&path).await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to get events: {}",
                response.msg
            )));
        }

        response
            .data
            .ok_or_else(|| BridgeError::Zulip("No events data in response".to_string()))
    }

    pub async fn subscribe_to_streams(
        &self,
        subscriptions: &[(&str, &str)],
    ) -> Result<()> {
        #[derive(serde::Serialize)]
        struct SubscribeRequest {
            subscriptions: String,
        }

        let subscriptions_json = serde_json::to_string(
            &subscriptions
                .iter()
                .map(|(name, desc)| {
                    serde_json::json!({
                        "name": name,
                        "description": desc
                    })
                })
                .collect::<Vec<_>>(),
        )
        .map_err(|e| BridgeError::Zulip(format!("Failed to serialize subscriptions: {}", e)))?;

        let request = SubscribeRequest {
            subscriptions: subscriptions_json,
        };

        let response: ZulipApiResponse<()> = self.post("users/me/subscriptions", &request).await?;

        if !response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to subscribe to streams: {}",
                response.msg
            )));
        }

        Ok(())
    }

    pub async fn upload_file(&self, file_path: &str) -> Result<String> {
        let url = self.api_url("user_uploads")?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            self.auth_header().parse().map_err(|e| BridgeError::Zulip(format!("Invalid header: {}", e)))?,
        );

        let file_content = std::fs::read(file_path)
            .map_err(|e| BridgeError::Io(e))?;

        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");

        let part = reqwest::multipart::Part::bytes(file_content)
            .file_name(file_name.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        let form = reqwest::multipart::Form::new().part("file", part);

        let response = self
            .client
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| BridgeError::Network(e.to_string()))?;

        debug!("Upload response status: {}, body: {}", status, body);

        #[derive(serde::Deserialize)]
        struct UploadResponse {
            uri: Option<String>,
        }

        let api_response: ZulipApiResponse<UploadResponse> =
            serde_json::from_str(&body).map_err(|e| {
                BridgeError::Zulip(format!("Failed to parse upload response: {}", e))
            })?;

        if !api_response.is_success() {
            return Err(BridgeError::Zulip(format!(
                "Failed to upload file: {}",
                api_response.msg
            )));
        }

        api_response
            .data
            .and_then(|d| d.uri)
            .ok_or_else(|| BridgeError::Zulip("No URI in upload response".to_string()))
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
