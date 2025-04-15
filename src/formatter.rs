use serde::Serialize;
use crate::error::{Result, Error};

/// Trait for formatting log entries
pub trait Formatter {
    /// Format a log entry into a string
    fn format<T: Serialize>(&self, entry: &T) -> Result<String>;
    
    /// Format a log entry with custom options
    fn format_with_options<T: Serialize>(&self, entry: &T, options: &FormatterOptions) -> Result<String>;
}

/// Options for formatting log entries
#[derive(Debug, Clone)]
pub struct FormatterOptions {
    /// Whether to include timestamps
    pub include_timestamps: bool,
    /// Whether to include log levels
    pub include_levels: bool,
    /// Whether to include metadata
    pub include_metadata: bool,
    /// Whether to include context
    pub include_context: bool,
    /// Whether to pretty-print the output
    pub pretty_print: bool,
}

impl Default for FormatterOptions {
    fn default() -> Self {
        Self {
            include_timestamps: true,
            include_levels: true,
            include_metadata: true,
            include_context: true,
            pretty_print: false,
        }
    }
}

/// Simple formatter that outputs JSON
pub struct SimpleFormatter;

impl SimpleFormatter {
    /// Create a new simple formatter
    pub fn new() -> Self {
        Self
    }
}

impl Default for SimpleFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for SimpleFormatter {
    fn format<T: Serialize>(&self, entry: &T) -> Result<String> {
        serde_json::to_string(entry).map_err(Error::SerializationError)
    }
    
    fn format_with_options<T: Serialize>(&self, entry: &T, options: &FormatterOptions) -> Result<String> {
        if options.pretty_print {
            serde_json::to_string_pretty(entry).map_err(Error::SerializationError)
        } else {
            serde_json::to_string(entry).map_err(Error::SerializationError)
        }
    }
}

/// Pretty formatter with more options
#[allow(dead_code)]
pub struct PrettyFormatter {
    options: FormatterOptions,
}

impl PrettyFormatter {
    /// Create a new pretty formatter
    pub fn new() -> Self {
        Self {
            options: FormatterOptions {
                pretty_print: true,
                ..Default::default()
            },
        }
    }
    
    /// Create with specific options
    pub fn with_options(options: FormatterOptions) -> Self {
        Self { options }
    }
}

impl Default for PrettyFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for PrettyFormatter {
    fn format<T: Serialize>(&self, entry: &T) -> Result<String> {
        serde_json::to_string_pretty(entry).map_err(Error::SerializationError)
    }
    
    fn format_with_options<T: Serialize>(&self, entry: &T, options: &FormatterOptions) -> Result<String> {
        if options.pretty_print {
            serde_json::to_string_pretty(entry).map_err(Error::SerializationError)
        } else {
            serde_json::to_string(entry).map_err(Error::SerializationError)
        }
    }
}