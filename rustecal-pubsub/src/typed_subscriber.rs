use crate::subscriber::Subscriber;
use crate::types::TopicId;
use rustecal_core::types::DataTypeInfo;
use rustecal_sys::{eCAL_SDataTypeInformation, eCAL_SReceiveCallbackData, eCAL_STopicId};
use std::{
    ffi::{CStr, c_void},
    marker::PhantomData,
    slice,
};

/// A trait for message types that can be deserialized by [`TypedSubscriber`].
///
/// Implement this trait for any type `T` that needs to be reconstructed
/// from a zero-copy byte slice plus metadata in a typed subscriber.
pub trait SubscriberMessage<'a>: Sized {
    /// Returns metadata (encoding, type name, descriptor) for this message type.
    fn datatype() -> DataTypeInfo;

    /// Deserializes a message instance from a zero-copy byte slice and its metadata.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A shared byte buffer containing the payload.
    /// * `data_type_info` - The corresponding `DataTypeInfo` describing the payload format.
    ///
    /// # Returns
    ///
    /// `Some(T)` on success, or `None` on failure.
    fn from_bytes(bytes: &'a [u8], data_type_info: &DataTypeInfo) -> Option<Self>;
}

/// A received message, with payload and metadata.
pub struct Received<T> {
    /// The deserialized payload of type `T`.
    pub payload: T,
    /// The topic name this message was received on.
    pub topic_name: String,
    /// The declared encoding format (e.g. "proto", "raw").
    pub encoding: String,
    /// The declared type name for the message.
    pub type_name: String,
    /// The publisher's send timestamp (microseconds since epoch).
    pub timestamp: i64,
    /// The publisher's logical clock at send time.
    pub clock: i64,
}

/// Wrapper to store a boxed callback for `Received<T>`
struct CallbackWrapper<'buf, T: SubscriberMessage<'buf>> {
    callback: Box<dyn Fn(Received<T>) + Send + Sync + 'static>,
    _phantom: PhantomData<&'buf T>,
}

impl<'buf, T: SubscriberMessage<'buf>> CallbackWrapper<'buf, T> {
    fn new<F>(f: F) -> Self
    where
        F: Fn(Received<T>) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(f),
            _phantom: PhantomData,
        }
    }

    fn call(&self, received: Received<T>) {
        (self.callback)(received);
    }
}

/// A type-safe, high-level subscriber for messages of type `T`.
///
/// Wraps a lower-level [`Subscriber`] and provides automatic deserialization
/// plus typed callbacks.
pub struct TypedSubscriber<'buf, T: SubscriberMessage<'buf>> {
    subscriber: Subscriber,
    user_data: *mut CallbackWrapper<'buf, T>,
    _phantom: PhantomData<&'buf T>,
}

impl<'buf, T: SubscriberMessage<'buf>> TypedSubscriber<'buf, T> {
    /// Creates a new typed subscriber for the specified topic.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The name of the topic to subscribe to.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if the subscriber was created successfully, or `Err` with a description.
    pub fn new(topic_name: &str) -> Result<Self, String> {
        let datatype = T::datatype();

        // dummy callback for construction
        let boxed = Box::new(CallbackWrapper::new(|_| {}));
        let user_data = Box::into_raw(boxed);

        let subscriber = Subscriber::new(topic_name, datatype, trampoline::<'buf, T>)?;
        Ok(Self {
            subscriber,
            user_data,
            _phantom: PhantomData,
        })
    }

    /// Registers a user callback that receives a deserialized message with metadata.
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(Received<T>) + Send + Sync + 'static,
    {
        // drop the old callback
        unsafe {
            let _ = Box::from_raw(self.user_data);
        }
        let boxed = Box::new(CallbackWrapper::new(callback));
        self.user_data = Box::into_raw(boxed);
        unsafe {
            rustecal_sys::eCAL_Subscriber_SetReceiveCallback(
                self.subscriber.raw_handle(),
                Some(trampoline::<'buf, T>),
                self.user_data as *mut _,
            );
        }
    }

    /// Returns the number of currently connected publishers.
    pub fn get_publisher_count(&self) -> usize {
        self.subscriber.get_publisher_count()
    }

    /// Returns the name of the subscribed topic.
    ///
    /// This is the same topic name passed to [`TypedSubscriber::new`].
    pub fn get_topic_name(&self) -> Option<String> {
        self.subscriber.get_topic_name()
    }

    /// Returns the topic ID assigned by eCAL.
    pub fn get_topic_id(&self) -> Option<TopicId> {
        self.subscriber.get_topic_id()
    }

    /// Returns the declared data type metadata for this topic.
    ///
    /// Includes:
    /// - `encoding` (e.g. `"proto"`, `"string"`, `"raw"`)
    /// - `type_name` (e.g. Protobuf type or Rust type)
    /// - `descriptor` (optional descriptor bytes, e.g. protobuf schema)
    pub fn get_data_type_information(&self) -> Option<DataTypeInfo> {
        self.subscriber.get_data_type_information()
    }
}

impl<'buf, T: SubscriberMessage<'buf>> Drop for TypedSubscriber<'buf, T> {
    /// Cleans up and removes the callback, releasing any boxed closures.
    fn drop(&mut self) {
        unsafe {
            rustecal_sys::eCAL_Subscriber_RemoveReceiveCallback(self.subscriber.raw_handle());
            let _ = Box::from_raw(self.user_data);
        }
    }
}

/// Internal trampoline for dispatching incoming messages to the registered user callback.
extern "C" fn trampoline<'buf, T: SubscriberMessage<'buf> + 'buf>(
    topic_id: *const eCAL_STopicId,
    data_type_info: *const eCAL_SDataTypeInformation,
    data: *const eCAL_SReceiveCallbackData,
    user_data: *mut c_void,
) {
    unsafe {
        if data.is_null() || user_data.is_null() {
            return;
        }

        // zero-copy view of the shared-memory payload
        let rd = &*data;
        let payload = slice::from_raw_parts(rd.buffer as *const u8, rd.buffer_size);

        // rebuild DataTypeInfo
        let info = &*data_type_info;
        let encoding = CStr::from_ptr(info.encoding).to_string_lossy().into_owned();
        let type_name = CStr::from_ptr(info.name).to_string_lossy().into_owned();
        let descriptor = if info.descriptor.is_null() || info.descriptor_length == 0 {
            Vec::new()
        } else {
            slice::from_raw_parts(info.descriptor as *const u8, info.descriptor_length).to_vec()
        };
        let dt_info = DataTypeInfo {
            encoding: encoding.clone(),
            type_name: type_name.clone(),
            descriptor,
        };

        // direct-borrow deserialization
        if let Some(decoded) = T::from_bytes(payload, &dt_info) {
            let cb_wrapper = &*(user_data as *const CallbackWrapper<'buf, T>);
            let topic_name = CStr::from_ptr((*topic_id).topic_name)
                .to_string_lossy()
                .into_owned();
            let received = Received {
                payload: decoded,
                topic_name,
                encoding: encoding.clone(),
                type_name: type_name.clone(),
                timestamp: rd.send_timestamp,
                clock: rd.send_clock,
            };
            cb_wrapper.call(received);
        }
    }
}
