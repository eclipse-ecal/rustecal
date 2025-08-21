//! Core lifecycle control for the eCAL runtime.
//!
//! This module provides safe Rust wrappers around initializing and finalizing
//! the eCAL communication system, as well as querying its state and version.
//!
//! The main entry point is the [`Ecal`] struct which provides:
//! - [`Ecal::initialize`] to start the middleware
//! - [`Ecal::finalize`] to shut it down
//! - [`Ecal::ok`] to query if eCAL is currently running
//! - [`Ecal::is_initialized`] and [`Ecal::is_component_initialized`] for introspection
//! - [`Ecal::version_string`], [`Ecal::version_date_string`] and [`Ecal::version_struct`] for version info
//!
//! Typically, you will call [`Ecal::initialize`] once at the beginning of your
//! application and [`Ecal::finalize`] at shutdown.

use std::ffi::{CStr, CString};
use std::ptr;

use crate::components::EcalComponents;
use crate::configuration::Configuration;
use crate::error::{check, RustecalError};
use crate::types::Version;

/// Provides access to the core initialization, shutdown, and state‑checking functions of eCAL.
pub struct Ecal;

impl Ecal {
    /// Initializes the eCAL runtime system.
    ///
    /// # Arguments
    ///
    /// * `unit_name` – Optional name to identify this process in eCAL.
    /// * `components` – Bitmask of which subsystems to enable.
    /// * `config` – Optional eCAL Configuration to use for initialization.
    ///
    /// # Errors
    ///
    /// Returns `Err(RustecalError::Ecal{..})` on any non‑zero C return code,
    /// or `RustecalError::Internal` if the unit name contains an interior NUL.
    pub fn initialize(
        unit_name: Option<&str>,
        components: EcalComponents,
        config: Option<&Configuration>,
    ) -> Result<(), RustecalError> {
        // Convert the unit name (if any), mapping CString errors
        let name: CString = if let Some(name) = unit_name {
            CString::new(name)
                .map_err(|e| RustecalError::Internal(format!("invalid unit name: {e}")))?
        } else {
            CString::new("").unwrap()
        };

        // Determine pointer to eCAL_Configuration (or null)
        let cfg_ptr = config
            .map(|c| c.as_ptr() as *mut rustecal_sys::eCAL_Configuration)
            .unwrap_or(ptr::null_mut());

        // Call the C API and map its return code
        let ret =
            unsafe { rustecal_sys::eCAL_Initialize(name.as_ptr(), &components.bits(), cfg_ptr) };
        check(ret)
    }

    /// Finalizes and shuts down the eCAL runtime system.
    ///
    /// After calling this, all publishers, subscribers, and services are invalidated.
    pub fn finalize() {
        unsafe { rustecal_sys::eCAL_Finalize() };
    }

    /// Returns `true` if the eCAL system is currently operational.
    pub fn ok() -> bool {
        unsafe { rustecal_sys::eCAL_Ok() != 0 }
    }

    /// Returns `true` if *any* eCAL components have been initialized.
    pub fn is_initialized() -> bool {
        unsafe { rustecal_sys::eCAL_IsInitialized() != 0 }
    }

    /// Returns `true` if the specified components are initialized.
    pub fn is_component_initialized(components: EcalComponents) -> bool {
        unsafe { rustecal_sys::eCAL_IsComponentInitialized(components.bits()) != 0 }
    }

    /// Returns the eCAL version string (e.g. `"6.0.0"`).
    ///
    /// This is infallible: if the C pointer is null or contains invalid UTF‑8,
    /// it returns `"unknown"`.
    pub fn version_string() -> &'static str {
        // SAFETY: eCAL guarantees a static, valid C string here.
        let ptr = unsafe { rustecal_sys::eCAL_GetVersionString() };
        if ptr.is_null() {
            "unknown"
        } else {
            unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("unknown") }
        }
    }

    /// Returns the eCAL build‑date string (e.g. `"01.05.2025"`).
    ///
    /// This is infallible: if the C pointer is null or contains invalid UTF‑8,
    /// it returns `"unknown"`.
    pub fn version_date_string() -> &'static str {
        let ptr = unsafe { rustecal_sys::eCAL_GetVersionDateString() };
        if ptr.is_null() {
            "unknown"
        } else {
            unsafe { CStr::from_ptr(ptr).to_str().unwrap_or("unknown") }
        }
    }

    /// Returns the eCAL version as a structured `Version { major, minor, patch }`.
    pub fn version_struct() -> Version {
        unsafe { rustecal_sys::eCAL_GetVersion().into() }
    }
}
