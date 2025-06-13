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
//!     let pub_ = TypedPublisher::<StringMessage>::new("example_topic").unwrap();
//!     pub_.send(&StringMessage("Hello!".into()));
//! }
//! ```
//!


// —————————————————————————————————————————————————————————————————————————————
// Core initialization & types (always available)
pub use rustecal_core::{Ecal, EcalComponents, Configuration};

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
    Publisher, Subscriber,
    // typed wrappers
    TypedPublisher, PublisherMessage,
    TypedSubscriber, SubscriberMessage,
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
    // server & client entrypoints
    ServiceServer, ServiceClient, ClientInstance,
    // request/response types
    ServiceRequest, ServiceResponse,
};

#[cfg(feature = "service")]
pub use rustecal_service::types::{
    // metadata & callback signature
    MethodInfo, ServiceCallback, CallState,
};
