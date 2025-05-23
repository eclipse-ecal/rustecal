//! # rustecal-types-serde
//!
//! A single, generic Serde-based message wrapper with runtime-selectable format
//! for eCAL Pub/Sub transport, dynamically using the encoding field from DataTypeInfo.
//!
//! ## Supported Formats
//! - JSON: via `serde_json`
//! - CBOR: via `serde_cbor`
//! - MessagePack: via `rmp-serde`

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_cbor;
use rmp_serde;
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;

/// Supported serialization formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Cbor,
    Msgpack,
}

impl Format {
    /// Parse a format from the encoding string in DataTypeInfo.
    pub fn from_encoding(enc: &str) -> Option<Self> {
        match enc.to_lowercase().as_str() {
            "json" => Some(Format::Json),
            "cbor" => Some(Format::Cbor),
            "msgpack" | "messagepack" => Some(Format::Msgpack),
            _ => None,
        }
    }
}

/// Generic Serde message with runtime format selection, based on DataTypeInfo.encoding.
#[derive(Debug, Clone)]
pub struct SerdeMessage<T> {
    /// The wrapped payload.
    pub data: Arc<T>,
    /// Selected format for (de)serialization.
    pub fmt: Format,
}

impl<T> SerdeMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Create a message with a chosen format.
    pub fn new_with_format(payload: T, fmt: Format) -> Self {
        SerdeMessage { data: Arc::new(payload), fmt }
    }

    /// Default constructor using JSON format.
    pub fn new(payload: T) -> Self {
        SerdeMessage { data: Arc::new(payload), fmt: Format::Json }
    }

    /// Serialize into bytes according to format.
    pub fn to_bytes_internal(&self) -> Vec<u8> {
        match self.fmt {
            Format::Json => serde_json::to_vec(&*self.data).expect("JSON serialize failed"),
            Format::Cbor => serde_cbor::to_vec(&*self.data).expect("CBOR serialize failed"),
            Format::Msgpack => rmp_serde::to_vec(&*self.data).expect("Msgpack serialize failed"),
        }
    }

    /// Deserialize bytes according to a specified format.
    pub fn from_bytes_with_format(bytes: Arc<[u8]>, fmt: Format) -> Option<Self> {
        let slice = bytes.as_ref();
        let payload = match fmt {
            Format::Json => serde_json::from_slice(slice).ok()?,
            Format::Cbor => serde_cbor::from_slice(slice).ok()?,
            Format::Msgpack => rmp_serde::from_slice(slice).ok()?,
        };
        Some(SerdeMessage { data: Arc::new(payload), fmt })
    }
}

impl<T> PublisherMessage for SerdeMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        // Expose the fmt in encoding so subscribers can parse
        DataTypeInfo { encoding: "serde".into(), type_name: std::any::type_name::<T>().into(), descriptor: vec![] }
    }

    fn to_bytes(&self) -> Arc<[u8]> {
        // Prepend a one-byte format tag (optional) or rely on subscriber reading DataTypeInfo
        Arc::from(self.to_bytes_internal())
    }
}

impl<T> SubscriberMessage for SerdeMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <SerdeMessage<T> as PublisherMessage>::datatype()
    }

    fn from_bytes(bytes: Arc<[u8]>, data_type_info: &DataTypeInfo) -> Option<Self> {
        // Determine format from DataTypeInfo.encoding
        let fmt = Format::from_encoding(&data_type_info.encoding)?;
        // Parse payload using determined format
        SerdeMessage::from_bytes_with_format(bytes, fmt)
    }
}
