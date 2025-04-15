# 🦋 ChrysalisRS

<div align="center">

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.65%2B-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/workflow/status/chrysalis-rs/chrysalis_rs/CI)](https://github.com/chrysalis-rs/chrysalis_rs/actions)

**Transform your logs into beautiful, structured JSON for elegant display in web UIs.**

[Getting Started](#getting-started) •
[Features](#features) •
[Examples](#examples) •
[Extending](#extending) •
[Documentation](https://docs.rs/chrysalis_rs) •
[Web UI Integration](#web-ui-integration) •
[Contributing](#contributing)

</div>

---

## ✨ Overview

ChrysalisRS transforms your application logs into rich, structured data ready for modern web interfaces. Like its namesake, it represents the beautiful metamorphosis of raw log data into elegant, interactive visualizations.

```rust
use chrysalis_rs::{LogEntry, LogLevel};

let mut log = LogEntry::new("User authentication successful", LogLevel::Info);
log.add_context("user_id", "12345");
log.add_context("ip_address", "192.168.1.1");

// Serialize to JSON with one line
let json = log.to_json()?;

// Ready for your web UI!
```

<div align="center">
<i>Simple to use, infinitely extensible.</i>
</div>

---

## 🚀 Getting Started

Add ChrysalisRS to your `Cargo.toml`:

```toml
[dependencies]
chrysalis_rs = "0.1.0"
```

### Quick Example

```rust
use chrysalis_rs::{LogEntry, LogLevel};

fn main() -> Result<(), chrysalis_rs::Error> {
    // Create a log entry
    let mut entry = LogEntry::new("Application started", LogLevel::Info);
    
    // Add context
    entry.add_context("version", "1.0.0")?;
    entry.add_context("environment", "production")?;
    
    // Add source location
    let entry = entry.with_source(file!(), line!());
    
    // Convert to JSON
    let json = entry.to_json()?;
    println!("{}", json);
    
    Ok(())
}
```

## ✨ Features

### Core Features

- **100% Extensible Architecture**: Design your own log structures with full serialization support
- **Zero Dependencies for Core Functions**: Minimal overhead, maximum performance
- **Type-Safe Logging**: Leverage Rust's type system for reliable logging
- **Rich Context**: Add structured metadata to every log entry
- **Customizable Serialization**: Control exactly what gets serialized
- **Web-First Design**: Optimized for modern visualization in UIs

### Advanced Features

- **Formatter Ecosystem**: Plugins for various output formats
- **Integration with Popular Logging Frameworks**: Works with `log`, `tracing`, and more
- **Extension System**: Add custom functionality without modifying core code
- **Adaptive Serialization**: Automatic handling of complex data types
- **Smart Sanitization**: Safe field names and values for JSON
- **Performance Optimized**: Built for high-throughput logging environments

## 📋 Examples

### Basic Usage

```rust
use chrysalis_rs::{LogEntry, LogLevel};

// Create a simple log entry
let mut entry = LogEntry::new("Database connection established", LogLevel::Info);

// Add structured context
entry.add_context("db_name", "users")?;
entry.add_context("connection_id", "conn-28734")?;

// Convert to JSON
let json = entry.to_json()?;
```

### Custom Log Types

```rust
use chrysalis_rs::Serializable;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ApiRequest {
    endpoint: String,
    method: String,
    status_code: u16,
    response_time_ms: u64,
    user_agent: String,
}

// Serializable is automatically implemented for Serialize types
let request = ApiRequest {
    endpoint: "/api/users".to_string(),
    method: "GET".to_string(),
    status_code: 200,
    response_time_ms: 45,
    user_agent: "Mozilla/5.0".to_string(),
};

// One-line serialization
let json = request.to_json()?;
```

### Integration with the log Crate

```rust
use chrysalis_rs::{LogAdapter, Adapter};
use log::{info, warn, error};

// Initialize the adapter
let adapter = LogAdapter::new();

// Use the standard log macros
info!("Application started");
warn!("Configuration file not found, using defaults");
error!("Database connection failed");

// Collect and convert logs
let records = // ... collect log records
for record in records {
    let entry = adapter.convert(&record)?;
    let json = entry.to_json()?;
    // Send to web UI, store in database, etc.
}
```

## 🧩 Extending

ChrysalisRS is designed for extension. Create custom log types, formatters, and more:

### Custom Extensions

```rust
use chrysalis_rs::{Extension, ExtensionRegistry};
use std::any::Any;

struct MetricsExtension {
    enabled: bool,
    log_count: usize,
}

impl Extension for MetricsExtension {
    fn name(&self) -> &str { "metrics" }
    
    fn initialize(&mut self) -> Result<(), chrysalis_rs::Error> {
        // Setup code here
        Ok(())
    }
    
    // Other required methods...
}

// Register your extension
let mut registry = ExtensionRegistry::new();
registry.register(MetricsExtension { enabled: true, log_count: 0 })?;
```

### Custom Formatters

```rust
use chrysalis_rs::{Formatter, FormatterOptions};
use serde::Serialize;

struct CompactFormatter;

impl Formatter for CompactFormatter {
    fn format<T: Serialize>(&self, entry: &T) -> Result<String, chrysalis_rs::Error> {
        // Custom formatting logic here
    }
    
    fn format_with_options<T: Serialize>(
        &self, 
        entry: &T, 
        options: &FormatterOptions
    ) -> Result<String, chrysalis_rs::Error> {
        // Format with specific options
    }
}
```

## 🖥️ Web UI Integration

ChrysalisRS is optimized for web UI integration, making it easy to create interactive log viewers.

### WebSocket Example

```rust
// Server-side code
let web_log = WebUILog::from(log_entry);
ws.send(web_log.to_json()?)?;

// Client-side JavaScript
socket.onmessage = function(event) {
    const log = JSON.parse(event.data);
    
    // Render log with UI metadata
    const logElement = document.createElement('div');
    logElement.className = log.ui_meta.css_class;
    
    // Apply formatting based on log level
    if (log.level === "error") {
        logElement.classList.add('highlighted');
    }
    
    // Add to log container
    document.getElementById('logs').appendChild(logElement);
};
```

### UI-Specific Metadata

ChrysalisRS allows you to include UI-specific metadata with your logs:

```rust
// Create a log entry with UI metadata
let mut entry = LogEntry::new("Payment processed", LogLevel::Info);
entry.add_context("ui_expand", true)?;
entry.add_context("ui_highlight", false)?;
entry.add_context("ui_icon", "credit-card")?;
```

## 📊 Performance

ChrysalisRS is designed for high-performance logging environments:

- **Zero-copy when possible**: Minimizes memory allocations
- **Deferred serialization**: Only convert to JSON when needed
- **Efficient context storage**: Optimized for fast lookups
- **Minimal dependencies**: Core functionality has few external dependencies
- **Benchmark suite**: Continuous performance testing

## 📦 Cargo Features

ChrysalisRS provides optional features that can be enabled in your `Cargo.toml`:

```toml
[dependencies.chrysalis_rs]
version = "0.1.0"
features = ["tracing", "log", "async", "compression"]
```

Available features:
- **tracing**: Integration with the tracing crate
- **log**: Integration with the log crate
- **async**: Async support for non-blocking operations
- **compression**: Log compression for storage efficiency
- **metrics**: Built-in metrics collection
- **security**: Additional sanitization and security features

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development

```bash
# Clone the repository
git clone https://github.com/chrysalis-rs/chrysalis_rs.git
cd chrysalis_rs

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example simple_log
```

### Guidelines

- Follow the Rust API guidelines
- Add tests for new features
- Update documentation accordingly
- Maintain backward compatibility when possible

## 📄 License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

<div align="center">
<p>🦋 Transform your logs into beautiful visualizations 🦋</p>
</div>