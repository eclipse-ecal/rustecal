use crate::{publisher::{Publisher, Timestamp}, payload_writer::PayloadWriter, types::TopicId};
use rustecal_core::types::DataTypeInfo;
use std::{marker::PhantomData, sync::Arc};

/// A trait for message types that can be published via [`TypedPublisher`].
///
/// Implement this trait for any type `T` that needs to be serialized
/// and sent through eCAL's typed publisher API.
pub trait PublisherMessage {
    /// Returns metadata (encoding, type name, descriptor) for this message type.
    fn datatype() -> DataTypeInfo;

    /// Serializes the message into a shared, reference-counted byte buffer.
    fn to_bytes(&self) -> Arc<[u8]>;
}

/// A type-safe, high-level wrapper over an eCAL publisher for messages of type `T`.
///
/// Wraps an untyped [`Publisher`] and enforces that only compatible messages
/// (implementing [`PublisherMessage`]) are published.
///
/// # Examples
///
/// ```no_run
/// use rustecal::TypedPublisher;
/// use rustecal_types_string::StringMessage;
///
/// let pub_ = TypedPublisher::<StringMessage>::new("hello topic").unwrap();
/// pub_.send(&StringMessage{data: "Hello!".into()}, Timestamp::Auto);
/// ```
pub struct TypedPublisher<T: PublisherMessage> {
    publisher: Publisher,
    _phantom:  PhantomData<T>,
}

impl<T: PublisherMessage> TypedPublisher<T> {
    /// Creates a new typed publisher for the given topic.
    ///
    /// # Arguments
    ///
    /// * `topic_name` — The topic name to publish to.
    ///
    /// # Errors
    ///
    /// Returns an `Err(String)` if the underlying eCAL publisher could not be created.
    pub fn new(topic_name: &str) -> Result<Self, String> {
        let datatype  = T::datatype();
        let publisher = Publisher::new(topic_name, datatype)?;

        Ok(Self { publisher, _phantom: PhantomData })
    }

    /// Sends a message of type `T` to all connected subscribers.
    ///
    /// Serializes the message via [`PublisherMessage::to_bytes()`], and
    /// specifies when to timestamp (auto or custom).
    ///
    /// # Arguments
    ///
    /// * `message` — The typed message to send.
    /// * `timestamp` — When to timestamp the message.
    ///
    /// # Returns
    ///
    /// `true` on success, `false` on failure.
    pub fn send(&self, message: &T, timestamp: Timestamp) -> bool {
        let bytes = message.to_bytes();
        self.publisher.send(&bytes, timestamp)
    }

    /// Performs a zero-copy send using a [`PayloadWriter`].
    ///
    /// Bypasses an intermediate buffer for types (like `BytesMessage`)
    /// that implement `PayloadWriter`.
    ///
    /// # Arguments
    ///
    /// * `writer` — A mutable reference to a `PayloadWriter`.
    /// * `timestamp` — When to timestamp the message.
    ///
    /// # Returns
    ///
    /// `true` on success, `false` on failure.
    pub fn send_payload_writer<W: PayloadWriter>(
        &self,
        writer: &mut W,
        timestamp: Timestamp,
    ) -> bool {
        self.publisher.send_payload_writer(writer, timestamp)
    }

    /// Returns the number of currently connected subscribers.
    pub fn get_subscriber_count(&self) -> usize {
        self.publisher.get_subscriber_count()
    }

    /// Returns the name of the topic this publisher is bound to.
    pub fn get_topic_name(&self) -> Option<String> {
        self.publisher.get_topic_name()
    }

    /// Returns the topic ID assigned by eCAL.
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.publisher.get_topic_id()
    }

    /// Returns the declared data type metadata for this topic.
    ///
    /// Includes:
    /// - `encoding` (e.g. `"proto"`, `"string"`, `"raw"`)
    /// - `type_name` (e.g. Protobuf type or Rust type)
    /// - `descriptor` (optional descriptor bytes, e.g. protobuf schema)
    pub fn get_data_type_information(&self) -> Option<DataTypeInfo> {
        self.publisher.get_data_type_information()
    }
}
