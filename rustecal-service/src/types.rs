use rustecal_sys::*;

#[derive(Debug, Clone, Copy)]
pub enum CallState {
    None,
    Executed,
    Timeout,
    Failed,
    Unknown(i32),
}

impl CallState {
    pub fn is_success(&self) -> bool {
        matches!(self, CallState::Executed)
    }
}

impl From<i32> for CallState {
    fn from(value: i32) -> Self {
        match value {
            x if x == rustecal_sys::eCAL_eCallState_eCAL_eCallState_none as i32 => CallState::None,
            x if x == rustecal_sys::eCAL_eCallState_eCAL_eCallState_executed as i32 => CallState::Executed,
            x if x == rustecal_sys::eCAL_eCallState_eCAL_eCallState_timeouted as i32 => CallState::Timeout,
            x if x == rustecal_sys::eCAL_eCallState_eCAL_eCallState_failed as i32 => CallState::Failed,
            other => CallState::Unknown(other),
        }
    }
}

impl From<u32> for CallState {
    fn from(value: u32) -> Self {
        CallState::from(value as i32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ServiceId {
    pub service_id: eCAL_SEntityId,
}

impl ServiceId {
    pub unsafe fn from_ffi(raw: &eCAL_SServiceId) -> Self {
        Self {
            service_id: raw.service_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServiceRequest {
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ServiceResponse {
    pub success: bool,
    pub server_id: ServiceId,
    pub error_msg: Option<String>,
    pub payload: Vec<u8>,
}

/// Metadata passed to method callbacks about the method interface.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub method_name: String,
    pub request_type: Option<String>,
    pub response_type: Option<String>,
}

/// The service callback signature used by ServiceServer.
///
/// Mimics the eCAL C++ API:
/// - Accepts `MethodInfo` and a reference to request bytes
/// - Returns response bytes (`Vec<u8>`)
pub type ServiceCallback = Box<dyn Fn(MethodInfo, &[u8]) -> Vec<u8> + Send + Sync + 'static>;
