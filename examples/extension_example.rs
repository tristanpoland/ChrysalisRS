#![allow(unused)]

use std::any::Any;
use chrysalis_rs::{
    Extension, ExtensionRegistry, LogEntry, LogLevel,
    error::Result,
};

// A simple extension that adds timestamp formatting options
struct TimestampFormatExtension {
    enabled: bool,
    format: String,
}

impl TimestampFormatExtension {
    fn new(format: impl Into<String>) -> Self {
        Self {
            enabled: true,
            format: format.into(),
        }
    }
    
    fn format_timestamp(&self, timestamp: chrono::DateTime<chrono::Utc>) -> String {
        timestamp.format(&self.format).to_string()
    }
    
    fn set_format(&mut self, format: impl Into<String>) {
        self.format = format.into();
    }
}

impl Extension for TimestampFormatExtension {
    fn name(&self) -> &str {
        "timestamp_formatter"
    }
    
    fn initialize(&mut self) -> Result<()> {
        // Validate the format string by attempting to format the current timestamp
        chrono::Utc::now().format(&self.format);
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() -> Result<()> {
    // Create our extension registry
    let mut registry = ExtensionRegistry::new();
    
    // Register our extension
    let timestamp_ext = TimestampFormatExtension::new("%Y-%m-%d %H:%M:%S");
    registry.register(timestamp_ext)?;
    
    // Initialize all extensions
    registry.initialize_all()?;
    
    // Create a log entry
    let entry = LogEntry::new("Testing extensions", LogLevel::Info);
    
    // Use our extension to format the timestamp
    if let Some(ext) = registry.get_by_type::<TimestampFormatExtension>() {
        if ext.is_enabled() {
            let formatted = ext.format_timestamp(entry.metadata.timestamp);
            println!("Log time: {}", formatted);
        }
    }
    
    // Get the extension by name and modify it
    if let Some(ext) = registry.get_mut("timestamp_formatter") {
        if let Some(ts_ext) = ext.as_any_mut().downcast_mut::<TimestampFormatExtension>() {
            ts_ext.set_format("%H:%M:%S");
            println!("Updated format!");
        }
    }
    
    // Use the modified extension
    if let Some(ext) = registry.get_by_type::<TimestampFormatExtension>() {
        let formatted = ext.format_timestamp(entry.metadata.timestamp);
        println!("Log time (new format): {}", formatted);
    }
    
    // Shutdown all extensions
    registry.shutdown_all()?;
    
    Ok(())
}