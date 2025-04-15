use std::marker::PhantomData;
use crate::core::{LogEntry, LogLevel};
use crate::error::Result;

/// Trait for adapting external logging systems to ChrysalisRS
pub trait Adapter<T> {
    /// Convert an external log type to a ChrysalisRS LogEntry
    fn convert(&self, external_log: &T) -> Result<LogEntry>;
    
    /// Configure the adapter with options
    fn configure(&mut self, options: AdapterOptions);
}

/// Options for adapters
#[derive(Debug, Clone)]
pub struct AdapterOptions {
    /// Whether to include source information
    pub include_source: bool,
    /// Whether to include thread information
    pub include_thread: bool,
    /// Whether to include stack traces for errors
    pub include_stack_traces: bool,
    /// Optional context extraction function (as string representation)
    pub context_extractor: Option<String>,
}

impl Default for AdapterOptions {
    fn default() -> Self {
        Self {
            include_source: true,
            include_thread: true,
            include_stack_traces: true,
            context_extractor: None,
        }
    }
}

/// Standard adapter for simple string logs
pub struct StandardAdapter<T> {
    options: AdapterOptions,
    _phantom: PhantomData<T>,
}

impl<T> StandardAdapter<T> {
    /// Create a new standard adapter
    pub fn new() -> Self {
        Self {
            options: AdapterOptions::default(),
            _phantom: PhantomData,
        }
    }
    
    /// Create with specific options
    pub fn with_options(options: AdapterOptions) -> Self {
        Self {
            options,
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for StandardAdapter<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: AsRef<str>> Adapter<T> for StandardAdapter<T> {
    fn convert(&self, external_log: &T) -> Result<LogEntry> {
        let message = external_log.as_ref().to_string();
        let entry = LogEntry::new(message, LogLevel::Info);
        Ok(entry)
    }
    
    fn configure(&mut self, options: AdapterOptions) {
        self.options = options;
    }
}