# 🦋 ChrysalisRS Extension Guide

This guide demonstrates how to extend ChrysalisRS to fit your specific needs. The core philosophy of ChrysalisRS is complete extensibility, allowing you to transform your logs into any structure required by your application or web UI.

## Table of Contents

- [Custom Log Structures](#custom-log-structures)
- [Custom Formatters](#custom-formatters)
- [Integration with Other Logging Systems](#integration-with-other-logging-systems)
- [Custom Extensions](#custom-extensions)
- [Web UI Integration](#web-ui-integration)
- [Advanced Use Cases](#advanced-use-cases)

## Custom Log Structures

ChrysalisRS allows you to create completely custom log structures while maintaining JSON serialization capabilities.

### Creating a Custom Log Type

Any struct that implements `serde::Serialize` and `serde::Deserialize` can be used with ChrysalisRS:

```rust
use serde::{Serialize, Deserialize};
use chrysalis_rs::Serializable; // This trait is automatically implemented for Serialize types

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseQueryLog {
    query: String,
    parameters: Vec<String>,
    execution_time_ms: u64,
    rows_affected: u64,
    success: bool,
    error_message: Option<String>,
}

impl DatabaseQueryLog {
    fn new(query: impl Into<String>, params: Vec<String>, time_ms: u64) -> Self {
        Self {
            query: query.into(),
            parameters: params,
            execution_time_ms: time_ms,
            rows_affected: 0,
            success: true,
            error_message: None,
        }
    }
    
    // Add other methods as needed
}

// Now you can use all Serializable methods:
let query_log = DatabaseQueryLog::new("SELECT * FROM users WHERE id = ?", vec!["42".to_string()], 15);
let json = query_log.to_json()?; // From the Serializable trait
```

### Nested Structures

You can create complex nested structures:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct QueryResult {
    column_names: Vec<String>,
    rows: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompleteQueryLog {
    query: DatabaseQueryLog,
    result: Option<QueryResult>,
    metadata: HashMap<String, String>,
}
```

## Custom Formatters

While JSON is the default format, you can create custom formatters for any output format.

### Implementing a Custom Formatter

```rust
use chrysalis_rs::{Formatter, FormatterOptions, Error};
use serde::Serialize;

struct XMLFormatter;

impl Formatter for XMLFormatter {
    fn format<T: Serialize>(&self, entry: &T) -> Result<String, Error> {
        // Implement XML conversion logic
        // For this example, we'll use a placeholder implementation
        let json = serde_json::to_string(entry)?;
        Ok(format!("<log>{}</log>", json))
    }
    
    fn format_with_options<T: Serialize>(&self, entry: &T, options: &FormatterOptions) -> Result<String, Error> {
        // Implement with options
        let mut result = self.format(entry)?;
        
        if options.pretty_print {
            // Add XML pretty printing
            result = format!("<pretty>\n  {}\n</pretty>", result);
        }
        
        Ok(result)
    }
}

// Usage:
let log = LogEntry::new("Test message", LogLevel::Info);
let formatter = XMLFormatter;
let xml = formatter.format(&log)?;
```

## Integration with Other Logging Systems

You can create adapters for any logging system by implementing the `Adapter` trait.

### Example for a Custom Logging System

```rust
use chrysalis_rs::{Adapter, LogEntry, LogLevel, Error};

// Your custom log type
struct MyCustomLog {
    text: String,
    severity: i32,
    timestamp: i64,
}

// Adapter for your custom logs
struct MyCustomAdapter;

impl Adapter<MyCustomLog> for MyCustomAdapter {
    fn convert(&self, external_log: &MyCustomLog) -> Result<LogEntry, Error> {
        // Map severity levels
        let level = match external_log.severity {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warn,
            _ => LogLevel::Error,
        };
        
        // Create the log entry
        let mut entry = LogEntry::new(external_log.text.clone(), level);
        
        // Add timestamp as context
        entry.add_context("original_timestamp", external_log.timestamp)?;
        
        Ok(entry)
    }
    
    fn configure(&mut self, _options: chrysalis_rs::AdapterOptions) {
        // Configure the adapter
    }
}
```

## Custom Extensions

Extensions allow you to add functionality to ChrysalisRS without modifying the core library.

### Creating a Custom Extension

```rust
use std::any::Any;
use chrysalis_rs::{Extension, Error};

struct MetricsExtension {
    enabled: bool,
    log_count: usize,
    level_counts: HashMap<LogLevel, usize>,
}

impl MetricsExtension {
    fn new() -> Self {
        Self {
            enabled: true,
            log_count: 0,
            level_counts: HashMap::new(),
        }
    }
    
    fn record_log(&mut self, level: LogLevel) {
        self.log_count += 1;
        *self.level_counts.entry(level).or_insert(0) += 1;
    }
    
    fn get_metrics(&self) -> HashMap<String, usize> {
        let mut metrics = HashMap::new();
        metrics.insert("total".to_string(), self.log_count);
        
        for (level, count) in &self.level_counts {
            metrics.insert(format!("{}", level), *count);
        }
        
        metrics
    }
}

impl Extension for MetricsExtension {
    fn name(&self) -> &str {
        "metrics"
    }
    
    fn initialize(&mut self) -> Result<(), Error> {
        // Reset counters
        self.log_count = 0;
        self.level_counts.clear();
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<(), Error> {
        // Maybe persist metrics
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

// Usage:
let mut registry = ExtensionRegistry::new();
registry.register(MetricsExtension::new())?;

// Later, when logging:
if let Some(metrics) = registry.get_mut_by_type::<MetricsExtension>() {
    metrics.record_log(LogLevel::Info);
}
```

## Web UI Integration

ChrysalisRS is designed for easy integration with web UIs.

### REST API Integration

```rust
async fn log_endpoint(log_entry: web::Json<LogEntry>) -> HttpResponse {
    match log_entry.to_json() {
        Ok(json) => {
            // Store the log in a database or send to a service
            HttpResponse::Ok().body(json)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}
```

### WebSocket Example

```rust
async fn websocket_handler(mut ws: WebSocket) -> Result<(), Error> {
    while let Some(msg) = ws.next().await {
        let msg = msg?;
        
        if let Ok(text) = msg.to_str() {
            // Parse the incoming JSON to a LogEntry
            if let Ok(entry) = serde_json::from_str::<LogEntry>(text) {
                // Process the log entry
                
                // Send a confirmation back
                let response = serde_json::json!({
                    "status": "received",
                    "id": entry.metadata.id
                });
                
                ws.send(response.to_string()).await?;
            }
        }
    }
    
    Ok(())
}
```

## Advanced Use Cases

### Custom Storage Backend

You can create a storage extension for persisting logs:

```rust
struct DatabaseStorage {
    connection: DbConnection,
    table_name: String,
}

impl DatabaseStorage {
    fn new(connection_string: &str, table: &str) -> Result<Self, Error> {
        // Connect to the database
        let connection = DbConnection::new(connection_string)?;
        
        Ok(Self {
            connection,
            table_name: table.to_string(),
        })
    }
    
    fn store_log(&self, entry: &LogEntry) -> Result<(), Error> {
        // Convert to JSON
        let json = entry.to_json()?;
        
        // Store in database
        self.connection.execute(
            &format!("INSERT INTO {} (id, timestamp, level, message, json) VALUES (?, ?, ?, ?, ?)",
                self.table_name),
            &[
                entry.metadata.id.to_string(),
                entry.metadata.timestamp.to_rfc3339(),
                entry.level.to_string(),
                entry.message.clone(),
                json,
            ],
        )?;
        
        Ok(())
    }
}
```

### Advanced Filtering and Processing

```rust
struct LogProcessor {
    filters: Vec<Box<dyn Fn(&LogEntry) -> bool>>,
    transformers: Vec<Box<dyn Fn(&mut LogEntry)>>,
}

impl LogProcessor {
    fn new() -> Self {
        Self {
            filters: Vec::new(),
            transformers: Vec::new(),
        }
    }
    
    fn add_filter<F>(&mut self, filter: F)
    where
        F: Fn(&LogEntry) -> bool + 'static,
    {
        self.filters.push(Box::new(filter));
    }
    
    fn add_transformer<F>(&mut self, transformer: F)
    where
        F: Fn(&mut LogEntry) + 'static,
    {
        self.transformers.push(Box::new(transformer));
    }
    
    fn process(&self, mut entry: LogEntry) -> Option<LogEntry> {
        // Apply all filters
        for filter in &self.filters {
            if !filter(&entry) {
                return None; // Log filtered out
            }
        }
        
        // Apply all transformers
        for transformer in &self.transformers {
            transformer(&mut entry);
        }
        
        Some(entry)
    }
}

// Usage:
let mut processor = LogProcessor::new();

// Add a filter for error logs only
processor.add_filter(|entry| matches!(entry.level, LogLevel::Error | LogLevel::Critical | LogLevel::Fatal));

// Add a transformer to add a timestamp
processor.add_transformer(|entry| {
    entry.add_context("processed_at", chrono::Utc::now().to_rfc3339()).unwrap();
});

// Process logs
if let Some(processed) = processor.process(log_entry) {
    // Use the processed log
}
```

By implementing these patterns, you can extend ChrysalisRS to handle virtually any logging scenario while maintaining a clean, extensible architecture.

---

This guide just scratches the surface of what's possible with ChrysalisRS. The library's flexibility and extensibility allow you to adapt it to any logging requirements while keeping your code clean and maintainable.