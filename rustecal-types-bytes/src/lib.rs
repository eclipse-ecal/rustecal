//! # rustecal-types-bytes
//!
//! Provides support for sending and receiving raw binary messages (`Vec<u8>`) with rustecal.

use std::{
    borrow::Cow,
    sync::Arc,
};
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;

/// A wrapper for raw‐binary messages used with typed eCAL pub/sub.
///
/// Internally holds either a borrowed slice (on receive) or an owned
/// `Arc<[u8]>` (on send).
pub struct BytesMessage<'a> {
    pub data: Cow<'a, [u8]>,
}

impl<'a> BytesMessage<'a> {
    /// Construct for sending: takes ownership of an `Arc<[u8]>`.
    pub fn owned(data: Arc<[u8]>) -> BytesMessage<'static> {
        BytesMessage { data: Cow::Owned(data.as_ref().to_vec()) }
    }
}

//
// SubscriberMessage: zero‐copy on receive
//
impl<'a> SubscriberMessage<'a> for BytesMessage<'a> {
    /// raw/bytes, no descriptor
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding:   "raw".into(),
            type_name:  "bytes".into(),
            descriptor: Vec::new(),
        }
    }

    /// On receive, we get a `&[u8]` slice straight from shared memory.
    fn from_bytes(bytes: &'a [u8], _info: &DataTypeInfo) -> Option<Self> {
        // zero‐copy: borrow the slice
        Some(BytesMessage { data: Cow::Borrowed(bytes) })
    }
}

//
// PublisherMessage: owns an Arc on send
//
impl<'a> PublisherMessage for BytesMessage<'a> {
    /// same metadata as above
    fn datatype() -> DataTypeInfo {
        <BytesMessage as SubscriberMessage>::datatype()
    }

    /// For send, convert into an `Arc<[u8]>` so eCAL’s zero‐copy writer
    /// can hand off the shared memory.  Note: this does copy *once*
    /// into a fresh Arc; if you’re doing *true* zero‐copy send,
    /// you’d use the PayloadWriter API instead of this path.
    fn to_bytes(&self) -> Arc<[u8]> {
        // if we’re already owned, reuse; otherwise clone the borrowed slice
        match &self.data {
            Cow::Owned(vec) => Arc::from(&vec[..]),
            Cow::Borrowed(s) => Arc::from(*s),
        }
    }
}
