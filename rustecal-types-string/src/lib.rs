//! # rustecal-types-string
//!
//! Provides support for sending and receiving `String` messages with rustecal.
//!
//! ## Example
//! ```rust
//! use std::sync::Arc;
//! use rustecal_types_string::StringMessage;
//! let msg = StringMessage(Arc::from("Hello World"));
//! ```

use std::str;
use std::sync::Arc;
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;

/// A wrapper for UTF-8 string messages used with typed eCAL pub/sub.
///
/// This type allows sending and receiving strings through the
/// `TypedPublisher` and `TypedSubscriber` APIs.
pub struct StringMessage {
    pub data: Arc<str>,
}

impl SubscriberMessage for StringMessage {
    /// Returns metadata describing this message type (`utf-8` encoded string).
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "utf-8".to_string(),
            type_name: "string".to_string(),
            descriptor: vec![],
        }
    }

    /// Attempts to decode a UTF-8 string from a byte buffer.
    fn from_bytes(bytes: Arc<[u8]>, _data_type_info: &DataTypeInfo) -> Option<Self> {
        str::from_utf8(bytes.as_ref())
            .ok()
            .map(|s| StringMessage{ data: Arc::<str>::from(s) })
    }
}

impl PublisherMessage for StringMessage {
    /// Returns the same metadata as [`SubscriberMessage::datatype`].
    fn datatype() -> DataTypeInfo {
        <StringMessage as SubscriberMessage>::datatype()
    }

    /// Serializes the string into a byte buffer.
    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(self.data.as_bytes())
    }
}
