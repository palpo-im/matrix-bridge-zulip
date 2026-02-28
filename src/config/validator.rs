use crate::config::Config;

#[derive(Debug, Clone)]
pub enum ConfigError {
    MissingField(String),
    InvalidValue(String),
    FileNotFound(String),
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
            ConfigError::FileNotFound(path) => write!(f, "Configuration file not found: {}", path),
            ConfigError::ParseError(msg) => write!(f, "Failed to parse configuration: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

pub fn validate_config(config: &Config) -> Result<(), ConfigError> {
    if config.bridge.homeserver_url.is_empty() {
        return Err(ConfigError::MissingField(
            "bridge.homeserver_url".to_string(),
        ));
    }

    if config.bridge.domain.is_empty() {
        return Err(ConfigError::MissingField("bridge.domain".to_string()));
    }

    if config.database.url.is_empty() {
        return Err(ConfigError::MissingField("database.url".to_string()));
    }

    if config.registration.appservice_token.is_empty() {
        return Err(ConfigError::MissingField(
            "registration.appservice_token".to_string(),
        ));
    }

    if config.registration.homeserver_token.is_empty() {
        return Err(ConfigError::MissingField(
            "registration.homeserver_token".to_string(),
        ));
    }

    if config.registration.sender_localpart.is_empty() {
        return Err(ConfigError::MissingField(
            "registration.sender_localpart".to_string(),
        ));
    }

    Ok(())
}
