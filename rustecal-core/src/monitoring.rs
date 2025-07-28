//! Runtime monitoring interface to query current system state from eCAL.
//!
//! This module wraps the C API `eCAL_Monitoring_GetMonitoring` and provides
//! a safe Rust API to access a snapshot of the middleware's state.

use crate::core_types::monitoring::{
    ClientInfo, MonitoringSnapshot, ProcessInfo, ServerInfo, TopicInfo,
};
use crate::error::RustecalError;
use std::{ptr, slice};

/// Provides access to eCAL runtime monitoring data.
pub struct Monitoring;

impl Monitoring {
    /// Retrieves a snapshot of the current system state from the eCAL runtime.
    ///
    /// If no eCAL instances are running (nothing to monitor), returns an
    /// empty `MonitoringSnapshot`. The C API signals “nothing to monitor”
    /// by returning a non‑zero status.
    ///
    /// # Errors
    ///
    /// - `RustecalError::NullPointer` if the C API returns a null pointer
    ///   when a snapshot *should* have been provided.
    pub fn get_snapshot() -> Result<MonitoringSnapshot, RustecalError> {
        // 1) Prepare a null pointer for the C function to fill in
        let mut raw: *mut rustecal_sys::eCAL_Monitoring_SMonitoring = ptr::null_mut();

        // 2) Call the FFI: non‑zero means “no snapshot available”
        let ret = unsafe { rustecal_sys::eCAL_Monitoring_GetMonitoring(&mut raw, ptr::null()) };

        // 3) If nothing to monitor, return an empty snapshot
        if ret != 0 {
            return Ok(MonitoringSnapshot {
                processes: Vec::new(),
                publishers: Vec::new(),
                subscribers: Vec::new(),
                servers: Vec::new(),
                clients: Vec::new(),
            });
        }

        // 4) On success (ret == 0), ensure we got a valid pointer
        if raw.is_null() {
            return Err(RustecalError::NullPointer);
        }

        // 5) Build the snapshot and free the C‑allocated memory
        let snapshot = unsafe {
            let processes = {
                let cnt = (*raw).processes_length;
                let ptr = (*raw).processes;
                slice::from_raw_parts(ptr, cnt)
                    .iter()
                    .map(|r| ProcessInfo::from(*r))
                    .collect()
            };

            let publishers = {
                let cnt = (*raw).publishers_length;
                let ptr = (*raw).publishers;
                slice::from_raw_parts(ptr, cnt)
                    .iter()
                    .map(|r| TopicInfo::from(*r))
                    .collect()
            };

            let subscribers = {
                let cnt = (*raw).subscribers_length;
                let ptr = (*raw).subscribers;
                slice::from_raw_parts(ptr, cnt)
                    .iter()
                    .map(|r| TopicInfo::from(*r))
                    .collect()
            };

            let servers = {
                let cnt = (*raw).servers_length;
                let ptr = (*raw).servers;
                slice::from_raw_parts(ptr, cnt)
                    .iter()
                    .map(|r| ServerInfo::from(*r))
                    .collect()
            };

            let clients = {
                let cnt = (*raw).clients_length;
                let ptr = (*raw).clients;
                slice::from_raw_parts(ptr, cnt)
                    .iter()
                    .map(|r| ClientInfo::from(*r))
                    .collect()
            };

            // free the C‑allocated snapshot
            rustecal_sys::eCAL_Free(raw as *mut _);

            MonitoringSnapshot {
                processes,
                publishers,
                subscribers,
                servers,
                clients,
            }
        };

        Ok(snapshot)
    }
}
