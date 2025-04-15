use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::error::{Result, Error};

/// Log levels supported by ChrysalisRS
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Trace level logging (lowest level)
    Trace,
    /// Debug level logging
    Debug,
    /// Information level logging
    Info,
    /// Warning level logging
    Warn,
    /// Error level logging
    Error,
    /// Critical error level logging
    Critical,
    /// Fatal error level logging (highest level)
    Fatal,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Critical => write!(f, "critical"),
            LogLevel::Fatal => write!(f, "fatal"),
        }
    }
}

/// Metadata for a log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    /// Unique ID for the log entry
    pub id: Uuid,
    /// Timestamp when the log was created
    pub timestamp: DateTime<Utc>,
    /// Source of the log (file, module, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Line number where the log originated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// Thread or task ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<String>,
    /// Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for MetaData {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: None,
            line: None,
            thread: None,
            custom: HashMap::new(),
        }
    }
}

impl MetaData {
    /// Create new metadata with default values
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Add a custom field to the metadata
    pub fn add_field<T>(&mut self, key: &str, value: T) -> Result<()>
    where
        T: Serialize,
    {
        let value = serde_json::to_value(value)
            .map_err(Error::SerializationError)?;
        self.custom.insert(key.to_string(), value);
        Ok(())
    }
}

/// Core log entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// The primary log message
    pub message: String,
    /// Log severity level
    pub level: LogLevel,
    /// Metadata about the log
    pub metadata: MetaData,
    /// Context fields for the log entry
    #[serde(default)]
    pub context: HashMap<String, serde_json::Value>,
}

impl LogEntry {
    /// Create a new log entry with the given message and level
    pub fn new(message: impl Into<String>, level: LogLevel) -> Self {
        Self {
            message: message.into(),
            level,
            metadata: MetaData::default(),
            context: HashMap::new(),
        }
    }
    
    /// Add context to the log entry
    pub fn add_context<T>(&mut self, key: impl Into<String>, value: T) -> Result<&mut Self>
    where
        T: Serialize,
    {
        let value = serde_json::to_value(value)
            .map_err(Error::SerializationError)?;
        self.context.insert(key.into(), value);
        Ok(self)
    }
    
    /// Add source location information
    pub fn with_source(mut self, file: &str, line: u32) -> Self {
        self.metadata.source = Some(file.to_string());
        self.metadata.line = Some(line);
        self
    }
    
    /// Add thread information
    pub fn with_thread(mut self, thread_id: impl Into<String>) -> Self {
        self.metadata.thread = Some(thread_id.into());
        self
    }
    
    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Error::SerializationError)
    }
    
    /// Convert to pretty-printed JSON string
    pub fn to_pretty_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Error::SerializationError)
    }
}

/// Trait for types that can be serialized to JSON
pub trait Serializable {
    /// Convert to JSON string
    fn to_json(&self) -> Result<String>;
    
    /// Convert to pretty-printed JSON string
    fn to_pretty_json(&self) -> Result<String>;
    
    /// Convert to a value that can be serialized
    fn to_value(&self) -> Result<serde_json::Value>;
}

impl<T> Serializable for T 
where
    T: Serialize,
{
    fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Error::SerializationError)
    }
    
    fn to_pretty_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Error::SerializationError)
    }
    
    fn to_value(&self) -> Result<serde_json::Value> {
        serde_json::to_value(self).map_err(Error::SerializationError)
    }
}