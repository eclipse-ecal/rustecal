//! # rustecal-types-protobuf
//!
//! Provides support for Protobuf message serialization with rustecal.

use prost::Message;
use prost_reflect::ReflectMessage;
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use std::sync::Arc;

/// Marker trait to opt-in a Protobuf type for use with eCAL.
///
/// This trait must be implemented for any `prost::Message` you wish to use
/// with `ProtobufMessage<T>`. It provides a type-level opt-in mechanism
/// to ensure users are aware of what's being exposed to eCAL.
pub trait IsProtobufType {}

/// A wrapper for protobuf messages used with typed eCAL pub/sub.
///
/// This type allows sending and receiving protobuf messages through the
/// `TypedPublisher` and `TypedSubscriber` APIs.
#[derive(Debug, Clone)]
pub struct ProtobufMessage<T> {
    pub data: Arc<T>,
}

impl<T> SubscriberMessage<'_> for ProtobufMessage<T>
where
    T: Message + Default + IsProtobufType + ReflectMessage,
{
    /// Returns metadata used by eCAL to describe the Protobuf type.
    ///
    /// This includes:
    /// - `proto` as encoding
    /// - the Rust type name
    /// - an optional descriptor (currently empty)
    fn datatype() -> DataTypeInfo {
        let default_instance = T::default();

        DataTypeInfo {
            encoding: "proto".to_string(),
            type_name: default_instance.descriptor().full_name().to_string(),
            descriptor: default_instance.descriptor().parent_pool().encode_to_vec(), // descriptor injection planned
        }
    }

    /// Decodes a Protobuf message from bytes.
    ///
    /// # Returns
    /// - `Some(ProtobufMessage<T>)` on success
    /// - `None` if decoding fails
    fn from_bytes(bytes: &[u8], _data_type_info: &DataTypeInfo) -> Option<Self> {
        T::decode(bytes).ok().map(|msg| ProtobufMessage {
            data: Arc::new(msg),
        })
    }
}

impl<T> PublisherMessage for ProtobufMessage<T>
where
    T: Message + Default + IsProtobufType + ReflectMessage,
{
    /// Returns the same datatype information as [`SubscriberMessage`] implementation.
    fn datatype() -> DataTypeInfo {
        <ProtobufMessage<T> as SubscriberMessage>::datatype()
    }

    /// Encodes the message to a byte buffer.
    ///
    /// # Panics
    /// Will panic if `prost::Message::encode` fails (should never panic for valid messages).
    fn to_bytes(&self) -> Arc<[u8]> {
        let mut buf = Vec::with_capacity(self.data.encoded_len());
        self.data
            .encode(&mut buf)
            .expect("Failed to encode protobuf message");
        Arc::from(buf)
    }
}
