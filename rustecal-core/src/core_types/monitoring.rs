//! Rust-safe wrappers for `eCAL_Monitoring_SMonitoring` and related types.
//!
//! These types represent the full monitoring snapshot of the eCAL runtime system.

use crate::types::DataTypeInfo;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Helper to safely convert C string pointers.
fn cstr(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}

/// Transport layer type.
#[derive(Debug, Clone)]
pub enum TransportLayerType {
    None,
    UdpMulticast,
    Shm,
    Tcp,
    Unknown(i32),
}

/// A single transport layer entry.
#[derive(Debug, Clone)]
pub struct TransportLayer {
    pub transport_type: TransportLayerType,
    pub version: i32,
    pub active: bool,
}

/// Full snapshot of monitoring information from the eCAL runtime.
#[derive(Debug, Clone)]
pub struct MonitoringSnapshot {
    pub processes: Vec<ProcessInfo>,
    pub publishers: Vec<TopicInfo>,
    pub subscribers: Vec<TopicInfo>,
    pub servers: Vec<ServerInfo>,
    pub clients: Vec<ClientInfo>,
}

/// A monitored eCAL process.
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub registration_clock: i32,
    pub host_name: String,
    pub shm_transport_domain: String,
    pub process_id: i32,
    pub process_name: String,
    pub unit_name: String,
    pub process_parameter: String,
    pub state_severity: i32,
    pub state_severity_level: i32,
    pub state_info: String,
    pub time_sync_state: i32,
    pub time_sync_module_name: String,
    pub component_init_state: i32,
    pub component_init_info: String,
    pub runtime_version: String,
    pub config_file_path: String,
}

/// A monitored topic (publisher or subscriber).
#[derive(Debug, Clone)]
pub struct TopicInfo {
    pub registration_clock: i32,
    pub host_name: String,
    pub shm_transport_domain: String,
    pub process_id: i32,
    pub process_name: String,
    pub unit_name: String,
    pub topic_id: i64,
    pub topic_name: String,
    pub direction: String,
    pub data_type: DataTypeInfo,
    pub transport_layers: Vec<TransportLayer>,
    pub topic_size: i32,
    pub connections_local: i32,
    pub connections_external: i32,
    pub message_drops: i32,
    pub data_id: i64,
    pub data_clock: i64,
    pub data_frequency: i32,
}

/// A method entry of a service.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub method_name: String,
    pub request_type: DataTypeInfo,
    pub response_type: DataTypeInfo,
    pub call_count: i64,
}

/// A monitored service server.
#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub registration_clock: i32,
    pub host_name: String,
    pub process_name: String,
    pub unit_name: String,
    pub process_id: i32,
    pub service_name: String,
    pub service_id: i64,
    pub version: u32,
    pub tcp_port_v0: u32,
    pub tcp_port_v1: u32,
    pub methods: Vec<MethodInfo>,
}

/// A monitored service client.
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub registration_clock: i32,
    pub host_name: String,
    pub process_name: String,
    pub unit_name: String,
    pub process_id: i32,
    pub service_name: String,
    pub service_id: i64,
    pub version: u32,
    pub methods: Vec<MethodInfo>,
}

// -----------------------------------------------------------------------------
// FFI Conversions
// -----------------------------------------------------------------------------

impl From<i32> for TransportLayerType {
    fn from(value: i32) -> Self {
        match value {
            0 => TransportLayerType::None,
            1 => TransportLayerType::UdpMulticast,
            4 => TransportLayerType::Shm,
            5 => TransportLayerType::Tcp,
            _ => TransportLayerType::Unknown(value),
        }
    }
}

impl From<u32> for TransportLayerType {
    fn from(value: u32) -> Self {
        TransportLayerType::from(value as i32)
    }
}

impl From<rustecal_sys::eCAL_Monitoring_STransportLayer> for TransportLayer {
    fn from(raw: rustecal_sys::eCAL_Monitoring_STransportLayer) -> Self {
        Self {
            transport_type: raw.type_.into(),
            version: raw.version,
            active: raw.active != 0,
        }
    }
}

impl From<rustecal_sys::eCAL_Monitoring_STopic> for TopicInfo {
    fn from(raw: rustecal_sys::eCAL_Monitoring_STopic) -> Self {
        let transport_layers = unsafe {
            std::slice::from_raw_parts(raw.transport_layer, raw.transport_layer_length)
                .iter()
                .cloned()
                .map(TransportLayer::from)
                .collect()
        };

        Self {
            registration_clock: raw.registration_clock,
            host_name: cstr(raw.host_name),
            shm_transport_domain: cstr(raw.shm_transport_domain),
            process_id: raw.process_id,
            process_name: cstr(raw.process_name),
            unit_name: cstr(raw.unit_name),
            topic_id: raw.topic_id,
            topic_name: cstr(raw.topic_name),
            direction: cstr(raw.direction),
            data_type: DataTypeInfo::from(raw.datatype_information),
            transport_layers,
            topic_size: raw.topic_size,
            connections_local: raw.connections_local,
            connections_external: raw.connections_external,
            message_drops: raw.message_drops,
            data_id: raw.data_id,
            data_clock: raw.data_clock,
            data_frequency: raw.data_frequency,
        }
    }
}

impl From<rustecal_sys::eCAL_Monitoring_SProcess> for ProcessInfo {
    fn from(raw: rustecal_sys::eCAL_Monitoring_SProcess) -> Self {
        Self {
            registration_clock: raw.registration_clock,
            host_name: cstr(raw.host_name),
            shm_transport_domain: cstr(raw.shm_transport_domain),
            process_id: raw.process_id,
            process_name: cstr(raw.process_name),
            unit_name: cstr(raw.unit_name),
            process_parameter: cstr(raw.process_parameter),
            state_severity: raw.state_severity,
            state_severity_level: raw.state_severity_level,
            state_info: cstr(raw.state_info),
            time_sync_state: raw.time_sync_state,
            time_sync_module_name: cstr(raw.time_sync_module_name),
            component_init_state: raw.component_init_state,
            component_init_info: cstr(raw.component_init_info),
            runtime_version: cstr(raw.ecal_runtime_version),
            config_file_path: cstr(raw.config_file_path),
        }
    }
}

impl From<rustecal_sys::eCAL_Monitoring_SMethod> for MethodInfo {
    fn from(raw: rustecal_sys::eCAL_Monitoring_SMethod) -> Self {
        Self {
            method_name: cstr(raw.method_name),
            request_type: DataTypeInfo::from(raw.request_datatype_information),
            response_type: DataTypeInfo::from(raw.response_datatype_information),
            call_count: raw.call_count,
        }
    }
}

impl From<rustecal_sys::eCAL_Monitoring_SServer> for ServerInfo {
    fn from(raw: rustecal_sys::eCAL_Monitoring_SServer) -> Self {
        let methods = unsafe {
            std::slice::from_raw_parts(raw.methods, raw.methods_length)
                .iter()
                .cloned()
                .map(MethodInfo::from)
                .collect()
        };

        Self {
            registration_clock: raw.registration_clock,
            host_name: cstr(raw.host_name),
            process_name: cstr(raw.process_name),
            unit_name: cstr(raw.unit_name),
            process_id: raw.process_id,
            service_name: cstr(raw.service_name),
            service_id: raw.service_id,
            version: raw.version,
            tcp_port_v0: raw.tcp_port_v0,
            tcp_port_v1: raw.tcp_port_v1,
            methods,
        }
    }
}

impl From<rustecal_sys::eCAL_Monitoring_SClient> for ClientInfo {
    fn from(raw: rustecal_sys::eCAL_Monitoring_SClient) -> Self {
        let methods = unsafe {
            std::slice::from_raw_parts(raw.methods, raw.methods_length)
                .iter()
                .cloned()
                .map(MethodInfo::from)
                .collect()
        };

        Self {
            registration_clock: raw.registration_clock,
            host_name: cstr(raw.host_name),
            process_name: cstr(raw.process_name),
            unit_name: cstr(raw.unit_name),
            process_id: raw.process_id,
            service_name: cstr(raw.service_name),
            service_id: raw.service_id,
            version: raw.version,
            methods,
        }
    }
}
