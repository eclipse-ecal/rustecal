//! # rustecal
//!
//! High-level entry point for Rust bindings to the [eCAL](https://github.com/eclipse-ecal/ecal) middleware.
//! Re-exports core pub/sub and service modules for user convenience.
//!
//! ## Modules
//! - `core`: Initialization and runtime management.
//! - `pubsub`: Typed publishers and subscribers.
//! - `service`: Synchronous RPC communication.
//!
//! ## Example
//! ```rust
//! use rustecal::{Ecal, TypedPublisher};
//! use rustecal_types_string::StringMessage;
//!
//! fn main() {
//!     Ecal::initialize(Some("example node"), EcalComponents::DEFAULT, None).unwrap();
//!     let pub_ = TypedPublisher::<StringMessage>::new("hello topic").unwrap();
//!     pub_.send(&StringMessage{data: "Hello!".into()}, Timestamp::Auto);
//! }
//! ```
//!

// —————————————————————————————————————————————————————————————————————————————
// Core initialization & types (always available)
pub use rustecal_core::{Configuration, Ecal, EcalComponents};

// —————————————————————————————————————————————————————————————————————————————
// Pub/Sub API (requires the `pubsub` feature)
#[cfg(feature = "pubsub")]
pub mod pubsub {
    //! Typed and untyped Publisher/Subscriber
    pub use rustecal_pubsub::*;
}

#[cfg(feature = "pubsub")]
pub use rustecal_pubsub::{
    // low‑level handles
    Publisher,
    PublisherMessage,
    Subscriber,
    SubscriberMessage,
    // typed wrappers
    TypedPublisher,
    TypedSubscriber,
};

// —————————————————————————————————————————————————————————————————————————————
// Service RPC API (requires the `service` feature)
#[cfg(feature = "service")]
pub mod service {
    //! RPC server & client, plus shared types
    pub use rustecal_service::*;
}

#[cfg(feature = "service")]
pub use rustecal_service::{
    ClientInstance,
    ServiceClient,
    // request/response types
    ServiceRequest,
    ServiceResponse,
    // server & client entrypoints
    ServiceServer,
};

#[cfg(feature = "service")]
pub use rustecal_service::types::{
    CallState,
    // metadata & callback signature
    MethodInfo,
    ServiceCallback,
};
