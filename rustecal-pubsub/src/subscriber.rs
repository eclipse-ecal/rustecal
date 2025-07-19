use crate::types::TopicId;
use rustecal_core::types::DataTypeInfo;
use rustecal_sys::*;
use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::ptr;

/// A safe and ergonomic wrapper around the eCAL C subscriber API.
///
/// This struct provides a high-level interface for subscribing to messages from
/// a topic using eCAL. It manages the lifecycle of the underlying eCAL subscriber handle
/// and allows registration of low-level C-compatible receive callbacks.
pub struct Subscriber {
    handle: *mut eCAL_Subscriber,
    _encoding: CString,
    _type_name: CString,
    _descriptor: Vec<u8>,
}

impl Subscriber {
    /// Creates a new subscriber and assigns a receive callback.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The name of the topic to subscribe to.
    /// * `data_type` - Metadata describing the expected message format (encoding, type name, descriptor).
    /// * `callback` - A raw extern "C" callback function invoked on message reception.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` on success or `Err(String)` on failure.
    pub fn new(
        topic_name: &str,
        data_type: DataTypeInfo,
        callback: extern "C" fn(
            *const eCAL_STopicId,
            *const eCAL_SDataTypeInformation,
            *const eCAL_SReceiveCallbackData,
            *mut c_void,
        ),
    ) -> Result<Self, String> {
        let c_topic = CString::new(topic_name).map_err(|_| "Invalid topic name")?;
        let c_encoding = CString::new(data_type.encoding).map_err(|_| "Invalid encoding")?;
        let c_type_name = CString::new(data_type.type_name).map_err(|_| "Invalid type name")?;

        let descriptor_ptr = if data_type.descriptor.is_empty() {
            ptr::null()
        } else {
            data_type.descriptor.as_ptr() as *const c_void
        };

        let data_type_info = eCAL_SDataTypeInformation {
            encoding: c_encoding.as_ptr(),
            name: c_type_name.as_ptr(),
            descriptor: descriptor_ptr,
            descriptor_length: data_type.descriptor.len(),
        };

        let handle =
            unsafe { eCAL_Subscriber_New(c_topic.as_ptr(), &data_type_info, None, ptr::null()) };

        if handle.is_null() {
            return Err("Failed to create eCAL_Subscriber".into());
        }

        unsafe { eCAL_Subscriber_SetReceiveCallback(handle, Some(callback), ptr::null_mut()) };

        Ok(Self {
            handle,
            _encoding: c_encoding,
            _type_name: c_type_name,
            _descriptor: data_type.descriptor,
        })
    }

    /// Returns the raw pointer to the underlying eCAL subscriber.
    ///
    /// This is primarily useful for advanced FFI use cases or low-level access.
    pub fn raw_handle(&self) -> *mut eCAL_Subscriber {
        self.handle
    }

    /// Retrieves the number of currently connected publishers.
    pub fn get_publisher_count(&self) -> usize {
        unsafe { eCAL_Subscriber_GetPublisherCount(self.handle) }
    }

    /// Retrieves the name of the topic this subscriber is connected to.
    ///
    /// # Returns
    ///
    /// The topic name as a `String`, or `None` if unavailable.
    pub fn get_topic_name(&self) -> Option<String> {
        unsafe {
            let raw = eCAL_Subscriber_GetTopicName(self.handle);
            if raw.is_null() {
                None
            } else {
                Some(CStr::from_ptr(raw).to_string_lossy().into_owned())
            }
        }
    }

    /// Retrieves the internal eCAL topic ID for this subscriber.
    ///
    /// # Returns
    ///
    /// A [`TopicId`] struct, or `None` if the information is unavailable.
    pub fn get_topic_id(&self) -> Option<TopicId> {
        unsafe {
            let raw = eCAL_Subscriber_GetTopicId(self.handle);
            if raw.is_null() {
                None
            } else {
                Some((*(raw as *const TopicId)).clone())
            }
        }
    }

    /// Retrieves the declared data type information for this subscriber.
    ///
    /// # Returns
    ///
    /// A [`DataTypeInfo`] object containing encoding, type name, and descriptor,
    /// or `None` if the metadata is unavailable.
    pub fn get_data_type_information(&self) -> Option<DataTypeInfo> {
        unsafe {
            let raw = eCAL_Subscriber_GetDataTypeInformation(self.handle);
            if raw.is_null() {
                return None;
            }

            let info = &*raw;

            let encoding = if info.encoding.is_null() {
                String::new()
            } else {
                CStr::from_ptr(info.encoding).to_string_lossy().into_owned()
            };

            let type_name = if info.name.is_null() {
                String::new()
            } else {
                CStr::from_ptr(info.name).to_string_lossy().into_owned()
            };

            let descriptor = if info.descriptor.is_null() || info.descriptor_length == 0 {
                vec![]
            } else {
                std::slice::from_raw_parts(info.descriptor as *const u8, info.descriptor_length)
                    .to_vec()
            };

            Some(DataTypeInfo {
                encoding,
                type_name,
                descriptor,
            })
        }
    }
}

impl Drop for Subscriber {
    /// Cleans up and removes the callback, releasing the underlying eCAL subscriber handle.
    fn drop(&mut self) {
        unsafe {
            eCAL_Subscriber_RemoveReceiveCallback(self.handle);
            eCAL_Subscriber_Delete(self.handle);
        }
    }
}
