pub mod types;

pub use types::{ZulipEvent, ZulipMessage, ZulipStream, ZulipUser};

pub struct ZulipClient {
    site: String,
    email: String,
    api_key: String,
    client: reqwest::Client,
}

impl ZulipClient {
    pub fn new(site: &str, email: &str, api_key: &str) -> Self {
        Self {
            site: site.to_string(),
            email: email.to_string(),
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_profile(&self) -> crate::utils::Result<ZulipUser> {
        todo!("Implement get_profile")
    }

    pub async fn get_stream_id(&self, stream_name: &str) -> crate::utils::Result<i64> {
        let _ = stream_name;
        todo!("Implement get_stream_id")
    }

    pub async fn send_message(
        &self,
        stream_id: i64,
        topic: &str,
        content: &str,
    ) -> crate::utils::Result<i64> {
        let _ = (stream_id, topic, content);
        todo!("Implement send_message")
    }

    pub async fn get_messages(
        &self,
        stream_id: i64,
        num_before: i32,
    ) -> crate::utils::Result<Vec<ZulipMessage>> {
        let _ = (stream_id, num_before);
        todo!("Implement get_messages")
    }
}
