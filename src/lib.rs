//! A simple and ergonomic webhook API for interfacing with the Discord webhook API.
//!
//! This crate provides a builder-based paradigm for constructing and sending messages and embeds
//! to Discord webhooks. It is designed for ease of use, allowing you to fluently build messages
//! and embeds with minimal boilerplate.
//!
//! ## Features
//! - Simple builder pattern for creating messages and embeds
//! - Supports both synchronous and asynchronous operations
//! - Asynchronous support is enabled when the `async` feature flag is turned on
//!
//! ## Example
//! ```no_run
//! use diswh::Webhook;
//! let webhook = Webhook::new("your_webhook_url");
//! // Send a message (async feature):
//! // webhook.send_message(message_packet).await?;
//! // Send a message (sync):
//! // webhook.send_message(message_packet)?;
//! ```

pub use prelude::*;

pub mod prelude;