use thiserror::Error;

/// Errors that can occur within ChrysalisRS operations
#[derive(Error, Debug)]
pub enum Error {
    /// Error when serializing to JSON
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// Error when using an extension
    #[error("Extension error: {0}")]
    ExtensionError(String),
    
    /// Error with log formatting
    #[error("Formatter error: {0}")]
    FormatterError(String),
    
    /// Generic error for other cases
    #[error("Log error: {0}")]
    LoggingError(String),
}

/// Result type for ChrysalisRS operations
pub type Result<T> = std::result::Result<T, Error>;
