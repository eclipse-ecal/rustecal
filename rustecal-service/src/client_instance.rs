use crate::response::ServiceResponse;
use crate::types::{ServiceId, ServiceRequest};
use rustecal_sys::*;
use std::ffi::CString;
use std::os::raw::c_void;

#[derive(Debug)]
pub struct ClientInstance {
    pub(crate) instance: *mut eCAL_ClientInstance,
}

impl ClientInstance {
    pub fn from_raw(raw: *mut eCAL_ClientInstance) -> Self {
        Self { instance: raw }
    }

    pub fn call(
        &self,
        method: &str,
        request: ServiceRequest,
        timeout_ms: Option<i32>,
    ) -> Option<ServiceResponse> {
        let c_method = CString::new(method).ok()?;
        let timeout_ptr = timeout_ms
            .as_ref()
            .map(|t| t as *const i32)
            .unwrap_or(std::ptr::null());

        let response_ptr = unsafe {
            eCAL_ClientInstance_CallWithResponse(
                self.instance,
                c_method.as_ptr(),
                request.payload.as_ptr() as *const c_void,
                request.payload.len(),
                timeout_ptr,
            )
        };

        if response_ptr.is_null() {
            return Some(ServiceResponse {
                success: false,
                server_id: ServiceId {
                    service_id: unsafe { std::mem::zeroed() },
                },
                error_msg: Some("call failed".into()),
                payload: vec![],
            });
        }

        unsafe {
            let response = &*response_ptr;
            let result = ServiceResponse::from_struct(response);
            eCAL_Free(response_ptr as *mut c_void);
            Some(result)
        }
    }
}
