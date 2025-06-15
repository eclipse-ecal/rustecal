//! # rustecal-pubsub
//!
//! Implements high-performance typed publish/subscribe communication over eCAL.
//!
//! ## Features
//! - Zero-copy shared memory support.
//! - Strongly-typed publishers and subscribers.
//! - Topic introspection and metadata.
//!
//! ## Key Types
//! - `TypedPublisher<T>`
//! - `TypedSubscriber<T>`
//! - Supported types: `StringMessage`, `BytesMessage`, `ProtobufMessage<T>`


// Re-export core init & types
pub use rustecal_core::{Ecal, EcalComponents};

// Sub‑modules
pub mod types;
pub mod publisher;
pub mod subscriber;
pub mod typed_publisher;
pub mod typed_subscriber;
pub mod payload_writer;

// Public API
pub use publisher::Publisher;
pub use subscriber::Subscriber;
pub use typed_publisher::TypedPublisher;
pub use typed_publisher::PublisherMessage;
pub use typed_subscriber::TypedSubscriber;
pub use typed_subscriber::SubscriberMessage;
pub use payload_writer::PayloadWriter;