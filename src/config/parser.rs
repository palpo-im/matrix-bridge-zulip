use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub bridge: BridgeConfig,
    pub database: DatabaseConfig,
    pub registration: RegistrationConfig,
    pub zulip: ZulipConfig,
    pub room: RoomConfig,
    pub limits: LimitsConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BridgeConfig {
    pub homeserver_url: String,
    pub domain: String,
    pub bind_address: String,
    pub port: u16,
    #[serde(default = "default_false")]
    pub disable_presence: bool,
    #[serde(default = "default_presence_interval")]
    pub presence_interval: u64,
}

fn default_false() -> bool {
    false
}

fn default_presence_interval() -> u64 {
    5000
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub db_type: DbType,
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    10
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DbType {
    Postgres,
    Sqlite,
    Mysql,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistrationConfig {
    pub bridge_id: String,
    pub sender_localpart: String,
    pub appservice_token: String,
    pub homeserver_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZulipConfig {
    #[serde(default = "default_puppet_separator")]
    pub puppet_separator: String,
    #[serde(default = "default_puppet_prefix")]
    pub puppet_prefix: String,
    #[serde(default = "default_member_sync")]
    pub member_sync: String,
    #[serde(default = "default_max_backfill")]
    pub max_backfill_amount: i32,
}

fn default_puppet_separator() -> String {
    "_".to_string()
}

fn default_puppet_prefix() -> String {
    "zulip_".to_string()
}

fn default_member_sync() -> String {
    "half".to_string()
}

fn default_max_backfill() -> i32 {
    100
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoomConfig {
    #[serde(default = "default_visibility")]
    pub default_visibility: String,
    #[serde(default)]
    pub room_alias_prefix: String,
}

fn default_visibility() -> String {
    "private".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct LimitsConfig {
    #[serde(default = "default_matrix_event_age_limit")]
    pub matrix_event_age_limit_ms: u64,
    #[serde(default)]
    pub room_count: i32,
}

fn default_matrix_event_age_limit() -> u64 {
    300000
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default)]
    pub file: Option<LoggingFileConfig>,
}

fn default_log_level() -> String {
    "info".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingFileConfig {
    pub path: String,
    #[serde(default)]
    pub rotate: bool,
}

impl Config {
    pub fn load(path: &str) -> crate::utils::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn validate(&self) -> crate::utils::Result<()> {
        if self.bridge.homeserver_url.is_empty() {
            return Err(crate::utils::BridgeError::Config(
                "homeserver_url is required".to_string(),
            ));
        }
        if self.bridge.domain.is_empty() {
            return Err(crate::utils::BridgeError::Config(
                "domain is required".to_string(),
            ));
        }
        if self.database.url.is_empty() {
            return Err(crate::utils::BridgeError::Config(
                "database url is required".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrganizationConfig {
    pub name: String,
    pub site: Option<String>,
    pub email: Option<String>,
    pub api_key: Option<String>,
    #[serde(default)]
    pub messages: HashMap<String, String>,
    #[serde(default = "default_max_backfill")]
    pub max_backfill_amount: i32,
}
