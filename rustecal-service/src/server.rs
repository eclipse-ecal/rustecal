use rustecal_sys::*;
use crate::types::{MethodInfo, ServiceCallback};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::{Arc, Mutex};

type SharedCallback = Arc<Mutex<HashMap<String, ServiceCallback>>>;

/// Represents a service server that can handle RPC-style requests.
pub struct ServiceServer {
    handle: *mut eCAL_ServiceServer,
    callbacks: SharedCallback,
}

impl ServiceServer {
    pub fn new(service_name: &str) -> Result<Self, String> {
        let c_service_name = CString::new(service_name).map_err(|_| "Invalid service name")?;

        let callbacks: SharedCallback = Arc::new(Mutex::new(HashMap::new()));
        let handle = unsafe { eCAL_ServiceServer_New(c_service_name.as_ptr(), None) };
        if handle.is_null() {
            return Err("Failed to create eCAL_ServiceServer".into());
        }

        Ok(Self {
            handle,
            callbacks,
        })
    }

    pub fn add_method(&mut self, method: &str, callback: ServiceCallback) -> Result<(), String> {
        let c_method = CString::new(method).map_err(|_| "Invalid method name")?;

        let mut method_info: eCAL_SServiceMethodInformation = unsafe { std::mem::zeroed() };
        method_info.method_name = c_method.as_ptr();

        self.callbacks
            .lock()
            .unwrap()
            .insert(method.to_string(), callback);

        let result = unsafe {
            eCAL_ServiceServer_SetMethodCallback(
                self.handle,
                &method_info,
                Some(Self::dispatch),
                Arc::as_ptr(&self.callbacks) as *mut c_void,
            )
        };

        if result != 0 {
            Err("Failed to register method callback".into())
        } else {
            Ok(())
        }
    }

    unsafe extern "C" fn dispatch(
        method_info: *const eCAL_SServiceMethodInformation,
        request_ptr: *const c_void,
        request_len: usize,
        response_ptr: *mut *mut c_void,
        response_len: *mut usize,
        user_data: *mut c_void,
    ) -> c_int {
        let callbacks = {
            let raw = user_data as *const Mutex<HashMap<String, ServiceCallback>>;
            unsafe { &*raw }.lock().unwrap()
        };

        let method_name = {
            if method_info.is_null() || unsafe { (*method_info).method_name }.is_null() {
                return 1;
            }

            let name_cstr = unsafe { CStr::from_ptr((*method_info).method_name) };
            match name_cstr.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => return 1,
            }
        };

        let request = if request_ptr.is_null() || request_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(request_ptr as *const u8, request_len) }
        };

        let info = MethodInfo {
            method_name: method_name.clone(),
            request_type: None,
            response_type: None,
        };

        let cb = match callbacks.get(&method_name) {
            Some(cb) => cb,
            None => return 1,
        };

        let response = cb(info, request);

        let buffer = unsafe { eCAL_Malloc(response.len()) };
        if buffer.is_null() {
            return 1;
        }

        unsafe {
            ptr::copy_nonoverlapping(response.as_ptr(), buffer as *mut u8, response.len());
            *response_ptr = buffer;
            *response_len = response.len();
        }

        0
    }
}

impl Drop for ServiceServer {
    fn drop(&mut self) {
        unsafe {
            eCAL_ServiceServer_Delete(self.handle);
        }
    }
}
