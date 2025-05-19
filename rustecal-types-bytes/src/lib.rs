//! # rustecal-types-bytes
//!
//! Provides support for sending and receiving raw binary messages (`Vec<u8>`) with rustecal.
//!
//! ## Example
//! ```rust
//! use std::sync::Arc;
//! use rustecal_types_bytes::BytesMessage;
//! let msg = BytesMessage(Arc::from([1, 2, 3, 4]));
//! ```

use std::sync::Arc;
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;

/// A wrapper for raw binary messages used with typed eCAL pub/sub.
///
/// This type allows sending and receiving raw binary payloads through the
/// `TypedPublisher` and `TypedSubscriber` APIs.
pub struct BytesMessage(pub Arc<[u8]>);

impl SubscriberMessage for BytesMessage {
    /// Returns metadata describing the message encoding and type.
    ///
    /// Encoding is `"raw"`, type name is `"bytes"`, and no descriptor is included.
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "raw".into(),
            type_name: "bytes".into(),
            descriptor: vec![],
        }
    }

    /// Creates a `BytesMessage` from a raw byte slice.
    fn from_bytes(bytes: Arc<[u8]>) -> Option<Self> {
        Some(BytesMessage(Arc::from(bytes)))
    }
}

impl PublisherMessage for BytesMessage {
    /// Reuses the `SubscriberMessage::datatype()` implementation.
    fn datatype() -> DataTypeInfo {
        <BytesMessage as SubscriberMessage>::datatype()
    }

    /// Returns the internal binary data as an Arc<[u8]> for zero-copy transmission.
    fn to_bytes(&self) -> Arc<[u8]> {
        self.0.clone()
    }
}
