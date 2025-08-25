//! # rustecal-service
//!
//! Implements synchronous RPC-style service communication over eCAL.
//!
//! ## Functionality
//! - `ServiceClient`: send requests to one or many services.
//! - `ServiceServer`: host services, handle requests with callbacks.
//!
//! ## Example
//! '''rust
//! use rustecal_service::ServiceClient;
//! let client = ServiceClient::new("mirror_service").unwrap();
//! let response = client.call("Hello!".as_bytes(), std::time::Duration::from_millis(500));
//! '''

pub mod client;
pub mod client_instance;
pub mod response;
pub mod server;
pub mod types;

// Public API
pub use client::ServiceClient;
pub use client_instance::ClientInstance;
pub use server::ServiceServer;
pub use types::ServiceRequest;
pub use types::ServiceResponse;
