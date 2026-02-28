use std::sync::Arc;
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use tracing::{debug, error, info, warn};

use super::{RegisterQueueRequest, ZulipClient, ZulipEvent, ZulipEventsResponse};
use crate::utils::{BridgeError, Result};

const EVENT_POLL_INTERVAL_SECS: u64 = 5;
const RECONNECT_DELAY_SECS: u64 = 5;
const MAX_RECONNECT_ATTEMPTS: u32 = 10;

pub struct ZulipWebSocketClient {
    client: Arc<ZulipClient>,
    event_tx: mpsc::Sender<ZulipEvent>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl ZulipWebSocketClient {
    pub fn new(client: Arc<ZulipClient>, event_tx: mpsc::Sender<ZulipEvent>) -> Self {
        Self {
            client,
            event_tx,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub async fn start(&self) -> Result<()> {
        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);
        
        let mut attempts = 0;
        
        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            match self.run_event_loop().await {
                Ok(()) => {
                    info!("Zulip event loop ended normally");
                    break;
                }
                Err(e) => {
                    attempts += 1;
                    error!(
                        "Zulip event loop error (attempt {}/{}): {}",
                        attempts, MAX_RECONNECT_ATTEMPTS, e
                    );
                    
                    if attempts >= MAX_RECONNECT_ATTEMPTS {
                        return Err(BridgeError::Zulip(format!(
                            "Max reconnection attempts ({}) reached",
                            MAX_RECONNECT_ATTEMPTS
                        )));
                    }
                    
                    info!(
                        "Reconnecting in {} seconds...",
                        RECONNECT_DELAY_SECS
                    );
                    tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                }
            }
        }
        
        Ok(())
    }

    pub fn stop(&self) {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    async fn run_event_loop(&self) -> Result<()> {
        info!("Registering Zulip event queue...");
        
        let request = RegisterQueueRequest::default();
        let queue = self.client.register_event_queue(&request).await?;
        
        info!(
            "Registered event queue: queue_id={}, last_event_id={}",
            queue.queue_id, queue.last_event_id
        );

        let mut last_event_id = queue.last_event_id;
        let queue_id = queue.queue_id;

        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            match self
                .client
                .get_events(&queue_id, last_event_id)
                .await
            {
                Ok(response) => {
                    for event in response.events {
                        if let Some(id) = event.id {
                            last_event_id = id;
                        }
                        
                        if let Err(e) = self.event_tx.send(event).await {
                            error!("Failed to send event to channel: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to get events: {}", e);
                }
            }

            tokio::time::sleep(Duration::from_secs(EVENT_POLL_INTERVAL_SECS)).await;
        }

        Ok(())
    }
}

pub struct ZulipRealTimeClient {
    client: Arc<ZulipClient>,
    event_tx: mpsc::Sender<ZulipEvent>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl ZulipRealTimeClient {
    pub fn new(client: Arc<ZulipClient>, event_tx: mpsc::Sender<ZulipEvent>) -> Self {
        Self {
            client,
            event_tx,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub async fn start_websocket(&self) -> Result<()> {
        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);

        let ws_url = self.get_websocket_url()?;
        
        info!("Connecting to Zulip WebSocket: {}", ws_url);
        
        let mut attempts = 0;
        
        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            match self.connect_and_listen(&ws_url).await {
                Ok(()) => {
                    info!("WebSocket connection closed normally");
                }
                Err(e) => {
                    attempts += 1;
                    error!(
                        "WebSocket error (attempt {}/{}): {}",
                        attempts, MAX_RECONNECT_ATTEMPTS, e
                    );
                    
                    if attempts >= MAX_RECONNECT_ATTEMPTS {
                        return Err(e);
                    }
                    
                    tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                }
            }
        }
        
        Ok(())
    }

    fn get_websocket_url(&self) -> Result<String> {
        let site = &self.client.site;
        let email = &self.client.email;
        let api_key = &self.client.api_key;
        
        let url = url::Url::parse(site)
            .map_err(|e| BridgeError::Zulip(format!("Invalid site URL: {}", e)))?;
        
        let ws_scheme = match url.scheme() {
            "https" => "wss",
            "http" => "ws",
            _ => "wss",
        };
        
        let ws_url = format!(
            "{}://{}:{}/api/v1/events?api_key={}",
            ws_scheme,
            url.host_str().unwrap_or("localhost"),
            url.port().unwrap_or(if ws_scheme == "wss" { 443 } else { 80 }),
            api_key
        );
        
        Ok(ws_url)
    }

    async fn connect_and_listen(&self, ws_url: &str) -> Result<()> {
        let (ws_stream, _) = connect_async(ws_url)
            .await
            .map_err(|e| BridgeError::Network(format!("WebSocket connection failed: {}", e)))?;
        
        info!("Connected to Zulip WebSocket");
        
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            tokio::select! {
                msg = ws_receiver.next() => {
                    match msg {
                        Some(Ok(WsMessage::Text(text))) => {
                            if let Err(e) = self.handle_websocket_message(&text).await {
                                error!("Failed to handle WebSocket message: {}", e);
                            }
                        }
                        Some(Ok(WsMessage::Ping(data))) => {
                            let _ = ws_sender.send(WsMessage::Pong(data)).await;
                        }
                        Some(Ok(WsMessage::Close(_))) => {
                            info!("WebSocket close frame received");
                            break;
                        }
                        Some(Ok(WsMessage::Pong(_))) => {
                            debug!("WebSocket pong received");
                        }
                        Some(Err(e)) => {
                            error!("WebSocket error: {}", e);
                            break;
                        }
                        None => {
                            info!("WebSocket stream ended");
                            break;
                        }
                        _ => {}
                    }
                }
                
                _ = tokio::time::sleep(Duration::from_secs(30)) => {
                    debug!("Sending WebSocket heartbeat");
                    let _ = ws_sender.send(WsMessage::Ping(vec![])).await;
                }
            }
        }
        
        Ok(())
    }

    async fn handle_websocket_message(&self, text: &str) -> Result<()> {
        debug!("WebSocket message: {}", text);
        
        let events: ZulipEventsResponse = serde_json::from_str(text)
            .map_err(|e| BridgeError::Parse(format!("Failed to parse events: {}", e)))?;
        
        for event in events.events {
            if let Err(e) = self.event_tx.send(event).await {
                error!("Failed to send event to channel: {}", e);
            }
        }
        
        Ok(())
    }

    pub fn stop(&self) {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}
