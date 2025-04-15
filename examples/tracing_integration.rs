#![allow(unused)]
//! Example showing integration with the tracing crate

use chrysalis_rs::{Error, Extension, ExtensionRegistry, LogEntry, LogLevel};

use tracing::{info, warn, error, Level, span, field::Visit};
use std::{any::Any, collections::HashMap};

/// Extension for integrating with tracing
struct TracingExtension {
    enabled: bool,
    level_map: HashMap<Level, LogLevel>,
}

impl TracingExtension {
    fn new() -> Self {
        let mut level_map = HashMap::new();
        level_map.insert(Level::TRACE, LogLevel::Trace);
        level_map.insert(Level::DEBUG, LogLevel::Debug);
        level_map.insert(Level::INFO, LogLevel::Info);
        level_map.insert(Level::WARN, LogLevel::Warn);
        level_map.insert(Level::ERROR, LogLevel::Error);
        
        Self {
            enabled: true,
            level_map,
        }
    }
    
    fn convert_level(&self, level: &Level) -> LogLevel {
        self.level_map.get(level).copied().unwrap_or(LogLevel::Info)
    }
}

impl Extension for TracingExtension {
    fn name(&self) -> &str {
        "tracing_integration"
    }
    
    fn initialize(&mut self) -> Result<(), Error> {
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<(), Error> {
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

/// A visitor for extracting fields from tracing events
struct TracingVisitor {
    entry: LogEntry,
}

impl TracingVisitor {
    fn new(message: String, level: LogLevel) -> Self {
        Self {
            entry: LogEntry::new(message, level),
        }
    }
    
    fn into_entry(self) -> LogEntry {
        self.entry
    }
}

impl Visit for TracingVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let value_str = format!("{:?}", value);
        let _ = self.entry.add_context(field.name(), value_str);
    }
    
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        let _ = self.entry.add_context(field.name(), value);
    }
    
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        let _ = self.entry.add_context(field.name(), value);
    }
    
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        let _ = self.entry.add_context(field.name(), value);
    }
    
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        let _ = self.entry.add_context(field.name(), value);
    }
    
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create our tracing extension
    let mut registry = ExtensionRegistry::new();
    let tracing_ext = TracingExtension::new();
    registry.register(tracing_ext)?;
    
    // Initialize extensions
    registry.initialize_all()?;
    
    // Set up some tracing spans and events
    let app_span = span!(Level::INFO, "app", version = "1.0.0");
    let _enter = app_span.enter();
    
    info!(target: "api", endpoint = "/users", "API request received");
    warn!(status_code = 429, "Rate limit exceeded");
    error!(error_code = "DB_CONN_FAILED", "Database connection error");
    
    // Convert tracing events to ChrysalisRS logs
    let visitor = TracingVisitor::new(
        "API request received".to_string(),
        LogLevel::Info,
    );
    
    // In a real application, you would register a tracing subscriber
    // that creates LogEntry objects for each event and outputs them as JSON
    
    // Just for demonstration, let's create a log entry manually
    let mut entry = LogEntry::new("API request received", LogLevel::Info);
    entry.add_context("endpoint", "/users")?;
    entry.add_context("method", "GET")?;
    entry.add_context("status_code", 200)?;
    
    // Serialize to JSON
    let json = entry.to_json()?;
    println!("Tracing-style JSON log:");
    println!("{}", json);
    
    // Shutdown extensions
    registry.shutdown_all()?;
    
    Ok(())
}