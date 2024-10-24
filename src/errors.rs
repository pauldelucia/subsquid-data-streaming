#[derive(thiserror::Error, Debug)]
pub enum DataStreamError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Parsing error: {0}")]
    ParsingError(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(serde_json::Error),
}
