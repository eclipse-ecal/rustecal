//! Core logging functions for emitting messages through eCAL.
//!
//! This module wraps the C API from `ecal_c/log.h` and provides access to
//! logging at various severity levels, as well as retrieval of current log entries.

use crate::core_types::logging::LogMessage;
use crate::error::RustecalError;
use crate::log_level::LogLevel;
use std::{ffi::CString, ptr, slice};

/// Provides logging functions to emit and retrieve messages via the eCAL runtime.
pub struct Log;

impl Log {
    /// Emits a message to the eCAL logging system with a specified severity.
    ///
    /// Any interior NUL in `message` is replaced with `"<invalid UTF-8>"`.
    pub fn log(level: LogLevel, message: &str) {
        let cstr =
            CString::new(message).unwrap_or_else(|_| CString::new("<invalid UTF-8>").unwrap());

        unsafe {
            rustecal_sys::eCAL_Logging_Log(level.into(), cstr.as_ptr());
        }
    }

    /// Fetches all current log messages stored in the eCAL runtime.
    ///
    /// If there are no logs available, returns an empty `Vec`.
    ///
    /// # Errors
    ///
    /// - `RustecalError::NullPointer` if the C API returns a null pointer
    ///   when a snapshot *should* have been provided.
    pub fn get_logging() -> Result<Vec<LogMessage>, RustecalError> {
        // 1) Prepare a null pointer for the C function to fill in.
        let mut raw_ptr: *mut rustecal_sys::eCAL_Logging_SLogging = ptr::null_mut();

        // 2) Call the FFI: non-zero => “no logs available”
        let ret = unsafe { rustecal_sys::eCAL_Logging_GetLogging(&mut raw_ptr) };
        if ret != 0 {
            return Ok(Vec::new());
        }

        // 3) Ensure we got a valid pointer
        if raw_ptr.is_null() {
            return Err(RustecalError::NullPointer);
        }

        // 4) Build the Vec<LogMessage> and free the C‑allocated memory
        let logs = unsafe {
            let logging = &*raw_ptr;
            let raw_messages = logging.log_messages;
            let len = logging.log_messages_length;

            let entries = slice::from_raw_parts(raw_messages, len)
                .iter()
                .map(|msg| LogMessage::from(*msg))
                .collect();

            // free the C buffer
            rustecal_sys::eCAL_Free(raw_ptr as *mut _);

            entries
        };

        Ok(logs)
    }
}
