//! Types used by the pub/sub layer of eCAL.

use rustecal_core::types::EntityId;
use rustecal_sys::*;
use std::ffi::CStr;

/// Internal eCAL topic identifier, used by publishers and subscribers.
#[derive(Debug, Clone)]
pub struct TopicId {
    pub entity_id: EntityId,
    pub topic_name: String,
}

impl From<eCAL_STopicId> for TopicId {
    fn from(raw: eCAL_STopicId) -> Self {
        Self {
            entity_id: EntityId::from(raw.topic_id),
            topic_name: cstr_to_string(raw.topic_name),
        }
    }
}

/// Helper function to safely convert a null-terminated C string.
fn cstr_to_string(ptr: *const std::os::raw::c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}
