use crate::publisher::Publisher;
use rustecal_core::types::DataTypeInfo;
use crate::types::TopicId;
use std::sync::Arc;
use std::marker::PhantomData;

/// Trait for types that can be published via [`TypedPublisher`].
///
/// Implement this trait for any message type `T` that should be serialized and sent
/// through eCAL's typed publisher API.
///
/// # Required Methods
///
/// - [`datatype()`]: Returns metadata describing the encoding, type name,
///   and optional descriptor (e.g., Protobuf schema).
/// - [`to_bytes()`]: Serializes the message into a binary buffer.
pub trait PublisherMessage {
    /// Returns topic metadata for this message type.
    fn datatype() -> DataTypeInfo;

    /// Serializes the message into a byte buffer for transmission.
    fn to_bytes(&self) -> Arc<[u8]>;
}

/// Type-safe, high-level wrapper around an eCAL publisher for messages of type `T`.
///
/// This struct wraps an untyped [`Publisher`] and ensures that only compatible messages
/// are published. It automatically serializes values of type `T` using the
/// [`PublisherMessage`] trait implementation.
///
/// # Example
///
/// ```no_run
/// use rustecal::TypedPublisher;
/// use rustecal_types_string::StringMessage;
///
/// let pub_ = TypedPublisher::<StringMessage>::new("hello").unwrap();
/// pub_.send(&StringMessage(Arc::from("Hello World!")));
/// ```
pub struct TypedPublisher<T: PublisherMessage> {
    publisher: Publisher,
    _phantom: PhantomData<T>,
}

impl<T: PublisherMessage> TypedPublisher<T> {
    /// Creates a new typed publisher for the specified topic.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The topic name to publish to.
    ///
    /// # Errors
    ///
    /// Returns a `String` if the underlying eCAL publisher could not be created.
    pub fn new(topic_name: &str) -> Result<Self, String> {
        let datatype = T::datatype();
        let publisher = Publisher::new(topic_name, datatype)?;

        Ok(Self {
            publisher,
            _phantom: PhantomData,
        })
    }

    /// Sends a message of type `T` to all connected subscribers.
    ///
    /// The message is serialized using [`PublisherMessage::to_bytes()`].
    ///
    /// # Arguments
    ///
    /// * `message` - The typed message to send.
    pub fn send(&self, message: &T) {
        let bytes = message.to_bytes();
        self.publisher.send(&bytes);
    }

    /// Sends a message of type `T` with a custom timestamp (in microseconds).
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send.
    /// * `timestamp` - Custom timestamp to associate with the message.
    pub fn send_with_timestamp(&self, message: &T, timestamp: i64) {
        let bytes = message.to_bytes();
        self.publisher.send_with_timestamp(&bytes, timestamp);
    }

    /// Returns the number of currently connected subscribers.
    pub fn get_subscriber_count(&self) -> usize {
        self.publisher.get_subscriber_count()
    }

    /// Returns the name of the topic this publisher is bound to.
    pub fn get_topic_name(&self) -> Option<String> {
        self.publisher.get_topic_name()
    }

    /// Returns the topic ID as seen by the eCAL system.
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.publisher.get_topic_id()
    }

    /// Returns the declared message type metadata.
    pub fn get_data_type_information(&self) -> Option<DataTypeInfo> {
        self.publisher.get_data_type_information()
    }
}
