//! # ChrysalisRS
//!
//! Transform your logs into beautiful, structured JSON for elegant display in web UIs.
//!
//! ChrysalisRS provides a flexible, extensible logging framework that makes it easy to
//! serialize logs into JSON format for modern web interfaces.
//!
//! ## Core Features
//!
//! * 100% extensible architecture
//! * Full JSON serialization/deserialization
//! * Type-safe logging structures
//! * Integration with common logging frameworks
//! * Rich context and metadata support
//!
//! ## Example
//!
//! ```rust
//! use chrysalis_rs::{LogEntry, LogLevel};
//!
//! let mut log = LogEntry::new("User authentication successful", LogLevel::Info);
//! log.add_context("user_id", "12345");
//! log.add_context("ip_address", "192.168.1.1");
//!
//! // Serialize to JSON with one line
//! let json = log.to_json().unwrap();
//! ```

mod core;
pub mod error;
mod formatter;
mod adapter;
mod extensions;
mod util;

pub use core::{LogEntry, LogLevel, Serializable, MetaData};
pub use error::Error;
pub use formatter::{Formatter, SimpleFormatter, PrettyFormatter};
pub use adapter::{Adapter, StandardAdapter, AdapterOptions};
pub use extensions::{Extension, ExtensionRegistry};
