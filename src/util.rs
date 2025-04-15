#![allow(unused)]
//! Utility functions and helpers for ChrysalisRS
//!
//! This module provides various utility functions for working with logs,
//! formatting, sanitization, and other common operations.

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::Path;
use chrono::{DateTime, Utc, SecondsFormat};
use rand::rng;
use uuid::Uuid;
use serde_json::Value;

use crate::error::{Result, Error};
use crate::core::LogLevel;

/// Format a timestamp to ISO 8601 format with millisecond precision
pub fn format_timestamp(timestamp: &DateTime<Utc>) -> String {
    timestamp.to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// Format a timestamp to a custom format
pub fn format_timestamp_custom(timestamp: &DateTime<Utc>, format: &str) -> String {
    timestamp.format(format).to_string()
}

/// Generate a new UUID as a string
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a new UUID object
pub fn generate_uuid_obj() -> Uuid {
    Uuid::new_v4()
}

/// Get the current time as a DateTime<Utc>
pub fn current_time() -> DateTime<Utc> {
    Utc::now()
}

/// Get the current time as a Unix timestamp (seconds since epoch)
pub fn current_timestamp() -> i64 {
    Utc::now().timestamp()
}

/// Get the current time as a Unix timestamp with millisecond precision
pub fn current_timestamp_millis() -> i64 {
    Utc::now().timestamp_millis()
}

/// Convert a Unix timestamp to DateTime<Utc>
pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| Utc::now())
}

/// Convert milliseconds since epoch to DateTime<Utc>
pub fn millis_to_datetime(millis: i64) -> DateTime<Utc> {
    let secs = millis / 1000;
    let nanos = ((millis % 1000) * 1_000_000) as u32;
    DateTime::from_timestamp(secs, nanos).unwrap_or_else(|| Utc::now())
}

/// Sanitize a field name for safe JSON use
///
/// Replaces characters that might cause issues in JSON field names
/// or in log processing systems.
pub fn sanitize_field_name(name: &str) -> String {
    name.replace(['.', ' ', '/', '\\', ':', '?', '#', '[', ']', '@', '(', ')', '"', '\'', '='], "_")
}
/// Create a nested field path
///
/// Joins parent and child field names with a dot separator,
/// sanitizing both to ensure valid JSON paths.
pub fn nested_field_path(parent: &str, child: &str) -> String {
    format!("{}.{}", sanitize_field_name(parent), sanitize_field_name(child))
}

/// Truncate a string if it exceeds a maximum length
///
/// Adds an ellipsis to indicate truncation if needed.
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_length.saturating_sub(3)])
    }
}

/// Get the filename from a path
pub fn filename_from_path(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path)
        .to_string()
}

/// Extract the file extension from a path
pub fn file_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}

/// Convert a LogLevel to a simple numeric value
///
/// Useful for sorting or filtering logs by severity.
/// Higher numbers indicate more severe levels.
pub fn log_level_to_numeric(level: LogLevel) -> u8 {
    match level {
        LogLevel::Trace => 0,
        LogLevel::Debug => 1,
        LogLevel::Info => 2,
        LogLevel::Warn => 3,
        LogLevel::Error => 4,
        LogLevel::Critical => 5,
        LogLevel::Fatal => 6,
    }
}

/// Convert a string to a LogLevel, defaulting to Info if not recognized
pub fn string_to_log_level(level: &str) -> LogLevel {
    match level.to_lowercase().as_str() {
        "trace" => LogLevel::Trace,
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" | "warning" => LogLevel::Warn,
        "error" | "err" => LogLevel::Error,
        "critical" | "crit" => LogLevel::Critical,
        "fatal" => LogLevel::Fatal,
        _ => LogLevel::Info,
    }
}

/// Calculate the elapsed time between two timestamps in milliseconds
pub fn elapsed_millis(start: &DateTime<Utc>, end: &DateTime<Utc>) -> i64 {
    (end.timestamp_millis() - start.timestamp_millis()).max(0)
}

/// Calculate elapsed time since a timestamp until now
pub fn elapsed_since(start: &DateTime<Utc>) -> i64 {
    elapsed_millis(start, &Utc::now())
}

/// Format a duration as a human-readable string
pub fn format_duration(duration_ms: i64) -> String {
    let seconds = duration_ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    
    if days > 0 {
        format!("{}d {}h", days, hours % 24)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}.{:03}s", seconds, duration_ms % 1000)
    }
}

/// Safely get a value from a serde_json::Value by path
///
/// The path is a dot-separated string of field names.
pub fn get_nested_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;
    
    for part in parts {
        match current {
            Value::Object(map) => {
                match map.get(part) {
                    Some(val) => current = val,
                    None => return None,
                }
            },
            Value::Array(arr) => {
                if let Ok(index) = part.parse::<usize>() {
                    if index < arr.len() {
                        current = &arr[index];
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            },
            _ => return None,
        }
    }
    
    Some(current)
}

/// Create a deep-merged version of two JSON values
///
/// If there are conflicts, values from 'update' overwrite values from 'base'.
pub fn merge_json_values(base: &Value, update: &Value) -> Value {
    match (base, update) {
        (Value::Object(base_map), Value::Object(update_map)) => {
            let mut result = base_map.clone();
            
            for (k, v) in update_map {
                match result.get(k) {
                    Some(base_value) => {
                        result.insert(k.clone(), merge_json_values(base_value, v));
                    },
                    None => {
                        result.insert(k.clone(), v.clone());
                    },
                }
            }
            
            Value::Object(result)
        },
        (_, update_value) => update_value.clone(),
    }
}

/// Flatten a nested JSON object into a single-level map with dot notation for keys
pub fn flatten_json(value: &Value, prefix: &str) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let new_key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                
                match v {
                    Value::Object(_) => {
                        let nested = flatten_json(v, &new_key);
                        result.extend(nested);
                    },
                    Value::Array(arr) => {
                        for (i, item) in arr.iter().enumerate() {
                            let array_key = format!("{}[{}]", new_key, i);
                            match item {
                                Value::Object(_) => {
                                    let nested = flatten_json(item, &array_key);
                                    result.extend(nested);
                                },
                                _ => {
                                    result.insert(array_key, item.clone());
                                },
                            }
                        }
                        // Also include the full array
                        result.insert(new_key, v.clone());
                    },
                    _ => {
                        result.insert(new_key, v.clone());
                    },
                }
            }
        },
        _ => {
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), value.clone());
            }
        },
    }
    
    result
}

/// Create a structured JSON error message
pub fn json_error(message: &str, code: Option<&str>) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("error".to_string(), Value::String(message.to_string()));
    
    if let Some(error_code) = code {
        obj.insert("code".to_string(), Value::String(error_code.to_string()));
    }
    
    obj.insert("timestamp".to_string(), Value::String(format_timestamp(&Utc::now())));
    
    Value::Object(obj)
}

/// Check if a JSON value is empty (null, empty string, empty array, or empty object)
pub fn is_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _ => false,
    }
}

/// Return the stacktrace of the current execution point
#[cfg(debug_assertions)]
pub fn get_stacktrace() -> String {
    use std::backtrace::Backtrace;
    format!("{:?}", Backtrace::capture())
}

#[cfg(not(debug_assertions))]
pub fn get_stacktrace() -> String {
    "Stack traces only available in debug mode".to_string()
}

/// Calculate a simple hash of a string, useful for identifying logs
pub fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for c in s.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u64);
    }
    hash
}

/// Estimate the JSON size of a String
pub fn estimate_json_string_size(s: &str) -> usize {
    // Account for quotes and possible escaping
    s.len() + 2 + s.chars().filter(|&c| c == '\\' || c == '"' || c == '\n' || c == '\r' || c == '\t').count()
}

/// Convert a HashMap to a serde_json::Value
pub fn hashmap_to_json<T: serde::Serialize>(map: &HashMap<String, T>) -> Result<Value> {
    serde_json::to_value(map).map_err(Error::SerializationError)
}

/// Get a random log ID
///
/// Format: YYYY-MM-DD-RANDOM
/// Where RANDOM is 6 random alphanumeric characters.
pub fn random_log_id() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    let mut rng = rng();
    let random: String = (0..6)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    let now = Utc::now();
    format!("{}-{}", now.format("%Y%m%d"), random)
}

/// Measure execution time of a function
pub fn measure_time<F, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = SystemTime::now();
    let result = f();
    let duration = SystemTime::now().duration_since(start).unwrap_or(Duration::from_secs(0));
    (result, duration)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sanitize_field_name() {
        assert_eq!(sanitize_field_name("user.name"), "user_name");
        assert_eq!(sanitize_field_name("path/to/file"), "path_to_file");
        assert_eq!(sanitize_field_name("query?param=value"), "query_param_value");
        assert_eq!(sanitize_field_name("item[0]"), "item_0_");
        assert_eq!(sanitize_field_name("func(x)"), "func_x_");
    }
    
    #[test]
    fn test_nested_field_path() {
        assert_eq!(nested_field_path("user", "name"), "user.name");
        assert_eq!(nested_field_path("user.profile", "email"), "user_profile.email");
        assert_eq!(nested_field_path("config", "server.port"), "config.server_port");
    }
    
    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(truncate_string("hello world", 5), "he...");
        assert_eq!(truncate_string("abcdefghijklmnop", 10), "abcdefg...");
    }
    
    #[test]
    fn test_string_to_log_level() {
        assert_eq!(string_to_log_level("trace"), LogLevel::Trace);
        assert_eq!(string_to_log_level("DEBUG"), LogLevel::Debug);
        assert_eq!(string_to_log_level("Info"), LogLevel::Info);
        assert_eq!(string_to_log_level("WARNING"), LogLevel::Warn);
        assert_eq!(string_to_log_level("error"), LogLevel::Error);
        assert_eq!(string_to_log_level("CRITICAL"), LogLevel::Critical);
        assert_eq!(string_to_log_level("fatal"), LogLevel::Fatal);
        assert_eq!(string_to_log_level("unknown"), LogLevel::Info);
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(500), "0.500s");
        assert_eq!(format_duration(1500), "1.500s");
        assert_eq!(format_duration(65000), "1m 5s");
        assert_eq!(format_duration(3665000), "1h 1m");
        assert_eq!(format_duration(90000000), "1d 1h");
    }
    
    #[test]
    fn test_get_nested_value() {
        let json = serde_json::json!({
            "user": {
                "profile": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "roles": ["admin", "user"]
            }
        });
        
        assert_eq!(
            get_nested_value(&json, "user.profile.name"),
            Some(&Value::String("John Doe".to_string()))
        );
        assert_eq!(
            get_nested_value(&json, "user.roles.0"),
            Some(&Value::String("admin".to_string()))
        );
        assert_eq!(get_nested_value(&json, "user.unknown"), None);
    }
    
    #[test]
    fn test_merge_json_values() {
        let base = serde_json::json!({
            "name": "Original",
            "config": {
                "timeout": 30,
                "retries": 3
            }
        });
        
        let update = serde_json::json!({
            "name": "Updated",
            "config": {
                "timeout": 60
            }
        });
        
        let expected = serde_json::json!({
            "name": "Updated",
            "config": {
                "timeout": 60,
                "retries": 3
            }
        });
        
        assert_eq!(merge_json_values(&base, &update), expected);
    }
    
    #[test]
    fn test_flatten_json() {
        let json = serde_json::json!({
            "user": {
                "name": "John",
                "address": {
                    "city": "New York",
                    "zip": "10001"
                }
            },
            "tags": ["a", "b", "c"]
        });
        
        let flattened = flatten_json(&json, "");
        
        assert_eq!(flattened.get("user.name"), Some(&Value::String("John".to_string())));
        assert_eq!(flattened.get("user.address.city"), Some(&Value::String("New York".to_string())));
        assert_eq!(flattened.get("user.address.zip"), Some(&Value::String("10001".to_string())));
        assert_eq!(flattened.get("tags[0]"), Some(&Value::String("a".to_string())));
    }
    
    #[test]
    fn test_is_empty_value() {
        assert!(is_empty_value(&Value::Null));
        assert!(is_empty_value(&Value::String("".to_string())));
        assert!(is_empty_value(&Value::Array(vec![])));
        assert!(is_empty_value(&Value::Object(serde_json::Map::new())));
        
        assert!(!is_empty_value(&Value::Bool(false)));
        assert!(!is_empty_value(&Value::Number(1.into())));
        assert!(!is_empty_value(&Value::String("hello".to_string())));
        assert!(!is_empty_value(&Value::Array(vec![Value::Null])));
    }
    
    #[test]
    fn test_simple_hash() {
        let hash1 = simple_hash("hello");
        let hash2 = simple_hash("hello");
        let hash3 = simple_hash("world");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}