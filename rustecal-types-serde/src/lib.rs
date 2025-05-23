//! # rustecal-types-serde
//!
//! eCAL Pub/Sub support for Serde-enabled messages with per-format wrappers.
//!
//! ## Supported Formats
//! - JSON: via `serde_json`
//! - CBOR: via `serde_cbor`
//! - MessagePack: via `rmp-serde`

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;

/// Helper to extract the short Rust type name without module prefixes.
fn short_type_name<T>() -> String {
    let full = std::any::type_name::<T>();
    full.rsplit("::").next().unwrap_or(full).to_string()
}

/// JSON-only message wrapper.
#[derive(Debug, Clone)]
pub struct JsonMessage<T> {
    /// The inner payload.
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

impl<T> PublisherMessage for JsonMessage<T>
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

    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(serde_json::to_vec(&*self.data).expect("JSON serialization failed"))
    }
}

impl<T> SubscriberMessage for JsonMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <JsonMessage<T> as PublisherMessage>::datatype()
    }

    fn from_bytes(bytes: Arc<[u8]>, _data_type_info: &DataTypeInfo) -> Option<Self> {
        serde_json::from_slice(bytes.as_ref())
            .ok()
            .map(|payload| JsonMessage { data: Arc::new(payload) })
    }
}

/// CBOR-only message wrapper.
#[derive(Debug, Clone)]
pub struct CborMessage<T> {
    /// The inner payload.
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

impl<T> PublisherMessage for CborMessage<T>
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

    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(serde_cbor::to_vec(&*self.data).expect("CBOR serialization failed"))
    }
}

impl<T> SubscriberMessage for CborMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <CborMessage<T> as PublisherMessage>::datatype()
    }

    fn from_bytes(bytes: Arc<[u8]>, _data_type_info: &DataTypeInfo) -> Option<Self> {
        serde_cbor::from_slice(bytes.as_ref())
            .ok()
            .map(|payload| CborMessage { data: Arc::new(payload) })
    }
}

/// MessagePack-only message wrapper.
#[derive(Debug, Clone)]
pub struct MsgpackMessage<T> {
    /// The inner payload.
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

impl<T> PublisherMessage for MsgpackMessage<T>
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

    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(rmp_serde::to_vec(&*self.data).expect("MessagePack serialization failed"))
    }
}

impl<T> SubscriberMessage for MsgpackMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <MsgpackMessage<T> as PublisherMessage>::datatype()
    }

    fn from_bytes(bytes: Arc<[u8]>, _data_type_info: &DataTypeInfo) -> Option<Self> {
        rmp_serde::from_slice(bytes.as_ref())
            .ok()
            .map(|payload| MsgpackMessage { data: Arc::new(payload) })
    }
}
