// src/lib.rs
//! # rustecal-types-serde
//!
//! Provides support for sending and receiving any Serde-enabled messages with rustecal via Pub/Sub.
//!
//! ## Features
//! - Wraps any type that implements Serde Serialize/Deserialize.
//! - Implements both `MessageSupport` and Pub/Sub message traits for JSON transport.

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json;

use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use rustecal_pubsub::typed_publisher::PublisherMessage;

/// A generic message wrapper supporting any Serde-enabled type.
///
/// Wraps the payload in an `Arc` for efficient cloning and implements
/// JSON (de)serialization alongside the eCAL Pub/Sub API integration.
#[derive(Debug, Clone)]
pub struct SerdeMessage<T> {
    /// The inner payload of the message
    pub data: Arc<T>,
}

impl<T> SerdeMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    /// Creates a new message from a payload.
    pub fn new(payload: T) -> Self {
        SerdeMessage { data: Arc::new(payload) }
    }

    /// Serializes the payload into JSON bytes.
    pub fn to_json_bytes(&self) -> Vec<u8> {
        // Serialize the inner payload, not the Arc wrapper
        serde_json::to_vec(&*self.data).expect("Serialization should not fail")
    }

    /// Deserializes JSON bytes into a `SerdeMessage<T>`.
    pub fn from_json_bytes(data: &[u8]) -> Self {
        let payload: T = serde_json::from_slice(data).expect("Deserialization should not fail");
        SerdeMessage::new(payload)
    }
}

impl<T> SubscriberMessage for SerdeMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Returns metadata describing this message type (JSON-encoded Serde).
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "json".to_string(),
            type_name: std::any::type_name::<T>().to_string(),
            descriptor: vec![],
        }
    }

    /// Attempts to decode a JSON-serialized message from a byte buffer.
    fn from_bytes(bytes: Arc<[u8]>) -> Option<Self> {
        // Deserialize directly into payload and wrap
        serde_json::from_slice(bytes.as_ref())
            .ok()
            .map(SerdeMessage::new)
    }
}

impl<T> PublisherMessage for SerdeMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Returns the same metadata as [`SubscriberMessage::datatype`].
    fn datatype() -> DataTypeInfo {
        <SerdeMessage<T> as SubscriberMessage>::datatype()
    }

    /// Serializes the message into a JSON byte buffer.
    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(self.to_json_bytes())
    }
}
