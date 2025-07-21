use crate::types::{CallState, ServiceId};
use rustecal_sys::*;
use std::ffi::CStr;

/// Represents a structured response to a service request,
/// primarily used by clients to parse returned data.
#[derive(Debug, Clone)]
pub struct ServiceResponse {
    pub success: bool,
    pub server_id: ServiceId,
    pub error_msg: Option<String>,
    pub payload: Vec<u8>,
}

impl ServiceResponse {
    /// Parses a raw FFI struct into a safe Rust response object.
    pub fn from_struct(response: &eCAL_SServiceResponse) -> Self {
        let success = CallState::from(response.call_state).is_success();

        let server_id = unsafe { ServiceId::from_ffi(&response.server_id) };

        let error_msg = if response.error_msg.is_null() {
            None
        } else {
            Some(unsafe {
                CStr::from_ptr(response.error_msg)
                    .to_string_lossy()
                    .into_owned()
            })
        };

        let payload = if response.response.is_null() || response.response_length == 0 {
            vec![]
        } else {
            unsafe {
                std::slice::from_raw_parts(response.response as *const u8, response.response_length)
                    .to_vec()
            }
        };

        Self {
            success,
            server_id,
            error_msg,
            payload,
        }
    }
}
