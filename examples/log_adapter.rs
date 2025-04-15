//! Example showing a complete adapter for the log crate

use chrysalis_rs::{Adapter, AdapterOptions, LogEntry, LogLevel, Error};
use log::{Level, Record};

/// Adapter for the standard log crate
pub struct LogAdapter {
    options: AdapterOptions,
}

impl LogAdapter {
    /// Create a new log adapter
    pub fn new() -> Self {
        Self {
            options: AdapterOptions::default(),
        }
    }
    
    /// Create with specific options
    pub fn with_options(options: AdapterOptions) -> Self {
        Self { options }
    }
    
    /// Convert log level to ChrysalisRS log level
    fn convert_level(&self, level: Level) -> LogLevel {
        match level {
            Level::Error => LogLevel::Error,
            Level::Warn => LogLevel::Warn,
            Level::Info => LogLevel::Info,
            Level::Debug => LogLevel::Debug,
            Level::Trace => LogLevel::Trace,
        }
    }
}

impl Default for LogAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Adapter<Record<'_>> for LogAdapter {
    fn convert(&self, record: &Record) -> Result<LogEntry, Error> {
        // Create the basic log entry
        let mut entry = LogEntry::new(
            record.args().to_string(),
            self.convert_level(record.level()),
        );
        
        // Add source information if enabled
        if self.options.include_source {
            if let Some(file) = record.file() {
                entry = entry.with_source(file, record.line().unwrap_or(0));
            }
        }
        
        // Add module path as context
        if let Some(module_path) = record.module_path() {
            entry.add_context("module_path", module_path)?;
        }
        
        // Add target as context
        entry.add_context("target", record.target())?;
        
        Ok(entry)
    }
    
    fn configure(&mut self, options: AdapterOptions) {
        self.options = options;
    }
}

/// A custom logger implementation using ChrysalisRS
pub struct ChrysalisLogger {
    adapter: LogAdapter,
}

impl ChrysalisLogger {
    /// Create a new ChrysalisRS logger
    pub fn new() -> Self {
        Self {
            adapter: LogAdapter::new(),
        }
    }
    
    /// Initialize the logger as the global logger
    pub fn init() -> Result<(), log::SetLoggerError> {
        let logger = Box::new(Self::new());
        log::set_boxed_logger(logger)?;
        log::set_max_level(log::LevelFilter::Trace);
        Ok(())
    }
}

impl log::Log for ChrysalisLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true // Log everything
    }
    
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        
        // Convert the log record to our format
        match self.adapter.convert(record) {
            Ok(entry) => {
                // In a real application, you might send this to a file, a database,
                // or a web UI. For this example, we'll just print the JSON.
                match entry.to_json() {
                    Ok(json) => println!("{}", json),
                    Err(e) => eprintln!("Error serializing log entry: {}", e),
                }
            },
            Err(e) => eprintln!("Error converting log record: {}", e),
        }
    }
    
    fn flush(&self) {
        // Ensure all logs are written
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize our logger
    ChrysalisLogger::init()?;
    
    // Log some messages
    log::trace!("This is a trace message");
    log::debug!("Debugging information");
    log::info!("Application startup complete");
    log::warn!("Configuration file not found, using defaults");
    log::error!("Failed to connect to database: timeout");
    
    // Log with additional context
    log::info!(target: "api_server", "Server listening on http://localhost:8080");
    
    Ok(())
}