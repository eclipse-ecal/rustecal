use crate::subscriber::Subscriber;
use crate::types::TopicId;
use rustecal_core::types::DataTypeInfo;
use rustecal_sys::{eCAL_SDataTypeInformation, eCAL_SReceiveCallbackData, eCAL_STopicId};
use std::ffi::{c_void, CStr};
use std::sync::Arc;
use std::marker::PhantomData;
use std::slice;

/// A trait for message types that can be deserialized by [`TypedSubscriber`].
///
/// Implement this trait for any type `T` that needs to be reconstructed
/// from raw bytes plus metadata in a typed subscriber.
pub trait SubscriberMessage: Sized {
    /// Returns metadata (encoding, type name, descriptor) for this message type.
    fn datatype() -> DataTypeInfo;

    /// Deserializes a message instance from a byte buffer and its metadata.
    ///
    /// # Arguments
    ///
    /// * `bytes` — A shared byte buffer containing the payload.
    /// * `data_type_info` — The corresponding `DataTypeInfo` describing the payload format.
    ///
    /// # Returns
    ///
    /// `Some(T)` on success, or `None` on failure.
    fn from_bytes(bytes: Arc<[u8]>, data_type_info: &DataTypeInfo) -> Option<Self>;
}

/// A received message, with payload and metadata.
pub struct Received<T> {
    /// The deserialized payload of type `T`.
    pub payload:    T,
    /// The topic name this message was received on.
    pub topic_name: String,
    /// The declared encoding format (e.g. "proto", "raw").
    pub encoding:   String,
    /// The declared type name for the message.
    pub type_name:  String,
    /// The publisher's send timestamp (microseconds since epoch).
    pub timestamp:  i64,
    /// The publisher's logical clock at send time.
    pub clock:      i64,
}

/// Wrapper to store a boxed callback for `Received<T>`
struct CallbackWrapper<T: SubscriberMessage> {
    callback: Box<dyn Fn(Received<T>) + Send + Sync>,
}

impl<T: SubscriberMessage> CallbackWrapper<T> {
    fn new<F>(f: F) -> Self
    where
        F: Fn(Received<T>) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(f),
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
///
/// # Examples
///
/// ```no_run
/// use rustecal::TypedSubscriber;
/// use rustecal_types_string::StringMessage;
///
/// let mut sub = TypedSubscriber::<StringMessage>::new("topic").unwrap();
/// sub.set_callback(|msg| println!("Got: {}", msg.payload.0));
/// ```
pub struct TypedSubscriber<T: SubscriberMessage> {
    subscriber: Subscriber,
    user_data: *mut CallbackWrapper<T>,
    _phantom: PhantomData<T>,
}

impl<T: SubscriberMessage> TypedSubscriber<T> {
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

        // Set dummy callback for construction, real callback will be assigned later
        let boxed: Box<CallbackWrapper<T>> = Box::new(CallbackWrapper::new(|_| {}));
        let user_data = Box::into_raw(boxed);

        let subscriber = Subscriber::new(topic_name, datatype, trampoline::<T>)?;

        Ok(Self {
            subscriber,
            user_data,
            _phantom: PhantomData,
        })
    }

    /// Registers a user callback that receives a deserialized message with metadata.
    ///
    /// # Arguments
    ///
    /// * `callback` - A closure accepting a [`Received<T>`] message.
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(Received<T>) + Send + Sync + 'static,
    {
        unsafe {
            // Drop the old callback
            let _ = Box::from_raw(self.user_data);
        }

        let boxed = Box::new(CallbackWrapper::new(callback));
        self.user_data = Box::into_raw(boxed);

        unsafe {
            rustecal_sys::eCAL_Subscriber_SetReceiveCallback(
                self.subscriber.raw_handle(),
                Some(trampoline::<T>),
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

impl<T: SubscriberMessage> Drop for TypedSubscriber<T> {
    /// Cleans up and removes the callback, releasing any boxed closures.
    fn drop(&mut self) {
        unsafe {
            rustecal_sys::eCAL_Subscriber_RemoveReceiveCallback(self.subscriber.raw_handle());
            let _ = Box::from_raw(self.user_data);
        }
    }
}

/// Internal trampoline for dispatching incoming messages to the registered user closure.
///
/// Converts C FFI types into Rust-safe [`Received<T>`] values and passes them to the callback.
extern "C" fn trampoline<T: SubscriberMessage>(
    topic_id: *const eCAL_STopicId,
    data_type_info: *const eCAL_SDataTypeInformation,
    data: *const eCAL_SReceiveCallbackData,
    user_data: *mut c_void,
) {
    unsafe {
        if data.is_null() || user_data.is_null() {
            return;
        }
        // Raw payload buffer
        let msg_slice = slice::from_raw_parts((*data).buffer as *const u8, (*data).buffer_size);
        let msg_arc: Arc<[u8]> = Arc::from(msg_slice);
        // Build Rust DataTypeInfo from eCAL metadata
        let encoding = CStr::from_ptr((*data_type_info).encoding).to_string_lossy().into_owned();
        let type_name = CStr::from_ptr((*data_type_info).name).to_string_lossy().into_owned();
        let descriptor = if (*data_type_info).descriptor.is_null() || (*data_type_info).descriptor_length == 0 {
            Vec::new()
        } else {
            slice::from_raw_parts((*data_type_info).descriptor as *const u8, (*data_type_info).descriptor_length as usize).to_vec()
        };
        let dt_info = DataTypeInfo { encoding, type_name, descriptor };
        // Deserialize with access to datatype information
        if let Some(decoded) = T::from_bytes(msg_arc.clone(), &dt_info) {
            let cb_wrapper = &*(user_data as *const CallbackWrapper<T>);
            let topic_name = CStr::from_ptr((*topic_id).topic_name).to_string_lossy().into_owned();
            let metadata = Received {
                payload: decoded,
                topic_name,
                encoding: dt_info.encoding.clone(),
                type_name: dt_info.type_name.clone(),
                timestamp: (*data).send_timestamp,
                clock: (*data).send_clock,
            };
            cb_wrapper.call(metadata);
        }
    }
}
