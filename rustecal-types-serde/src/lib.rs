//! # rustecal-types-serde
//!
//! Support for sending and receiving any Serde-enabled messages via eCAL Pub/Sub.
//!
//! ## Supported Formats
//! - JSON: text-based, via `serde_json`
//! - CBOR: binary, via `serde_cbor`
//! - MessagePack: binary, via `rmp-serde`
//!
//! Provides dedicated wrappers for each format: `JsonMessage`, `CborMessage`, and `MsgpackMessage`.

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use rustecal_pubsub::typed_publisher::PublisherMessage;

/// Helper to extract the short Rust type name of `T`, without module paths.
fn short_type_name<T>() -> String {
    let full = std::any::type_name::<T>();
    full.rsplit("::").next().unwrap_or(full).to_string()
}

/// JSON-only message wrapper.
#[derive(Debug, Clone)]
pub struct JsonMessage<T> {
    /// The inner payload of the message.
    pub data: Arc<T>,
}

impl<T> JsonMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Create a new JSON message.
    pub fn new(payload: T) -> Self {
        JsonMessage { data: Arc::new(payload) }
    }
}

impl<T> SubscriberMessage for JsonMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "json".into(),
            type_name: short_type_name::<T>(),
            descriptor: vec![],
        }
    }

    fn from_bytes(bytes: Arc<[u8]>) -> Option<Self> {
        serde_json::from_slice(bytes.as_ref())
            .ok()
            .map(|payload| JsonMessage { data: Arc::new(payload) })
    }
}

impl<T> PublisherMessage for JsonMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <JsonMessage<T> as SubscriberMessage>::datatype()
    }

    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(serde_json::to_vec(&*self.data).expect("JSON serialization failed"))
    }
}

/// CBOR-only message wrapper.
#[derive(Debug, Clone)]
pub struct CborMessage<T> {
    /// The inner payload of the message.
    pub data: Arc<T>,
}

impl<T> CborMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Create a new CBOR message.
    pub fn new(payload: T) -> Self {
        CborMessage { data: Arc::new(payload) }
    }
}

impl<T> SubscriberMessage for CborMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "cbor".into(),
            type_name: short_type_name::<T>(),
            descriptor: vec![],
        }
    }

    fn from_bytes(bytes: Arc<[u8]>) -> Option<Self> {
        serde_cbor::from_slice(bytes.as_ref())
            .ok()
            .map(|payload| CborMessage { data: Arc::new(payload) })
    }
}

impl<T> PublisherMessage for CborMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <CborMessage<T> as SubscriberMessage>::datatype()
    }

    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(serde_cbor::to_vec(&*self.data).expect("CBOR serialization failed"))
    }
}

/// MessagePack-only message wrapper.
#[derive(Debug, Clone)]
pub struct MsgpackMessage<T> {
    /// The inner payload of the message.
    pub data: Arc<T>,
}

impl<T> MsgpackMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Create a new MessagePack message.
    pub fn new(payload: T) -> Self {
        MsgpackMessage { data: Arc::new(payload) }
    }
}

impl<T> SubscriberMessage for MsgpackMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "msgpack".into(),
            type_name: short_type_name::<T>(),
            descriptor: vec![],
        }
    }

    fn from_bytes(bytes: Arc<[u8]>) -> Option<Self> {
        rmp_serde::from_slice(bytes.as_ref())
            .ok()
            .map(|payload| MsgpackMessage { data: Arc::new(payload) })
    }
}

impl<T> PublisherMessage for MsgpackMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <MsgpackMessage<T> as SubscriberMessage>::datatype()
    }

    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(rmp_serde::to_vec(&*self.data).expect("MessagePack serialization failed"))
    }
}
