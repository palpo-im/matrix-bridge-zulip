use thiserror::Error;

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Matrix error: {0}")]
    Matrix(String),

    #[error("Zulip error: {0}")]
    Zulip(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Room not found: {0}")]
    RoomNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, BridgeError>;
