//! eCAL logging severity levels.
//!
//! This mirrors the C enum `eCAL_Logging_eLogLevel` from `ecal_c/log.h`:
//!
//! ```c
//! enum eCAL_Logging_eLogLevel
//! {
//!   eCAL_Logging_log_level_none    = 0,
//!   eCAL_Logging_log_level_all     = 255,
//!   eCAL_Logging_log_level_info    = 1,
//!   eCAL_Logging_log_level_warning = 2,
//!   eCAL_Logging_log_level_error   = 4,
//!   eCAL_Logging_log_level_fatal   = 8,
//!   eCAL_Logging_log_level_debug1  = 16,
//!   eCAL_Logging_log_level_debug2  = 32,
//!   eCAL_Logging_log_level_debug3  = 64,
//!   eCAL_Logging_log_level_debug4  = 128,
//! };
//! ```

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// No logging.
    None    = 0,
    /// Informational messages.
    Info    = 1,
    /// Warnings.
    Warning = 2,
    /// Errors.
    Error   = 4,
    /// Fatal errors.
    Fatal   = 8,
    /// Debug level 1.
    Debug1  = 16,
    /// Debug level 2.
    Debug2  = 32,
    /// Debug level 3.
    Debug3  = 64,
    /// Debug level 4.
    Debug4  = 128,
    /// All levels.
    All     = 255,
}

impl From<i32> for LogLevel {
    fn from(value: i32) -> Self {
        match value {
            1   => LogLevel::Info,
            2   => LogLevel::Warning,
            4   => LogLevel::Error,
            8   => LogLevel::Fatal,
            16  => LogLevel::Debug1,
            32  => LogLevel::Debug2,
            64  => LogLevel::Debug3,
            128 => LogLevel::Debug4,
            255 => LogLevel::All,
            _   => LogLevel::None,
        }
    }
}

impl From<u32> for LogLevel {
    fn from(value: u32) -> Self {
        LogLevel::from(value as i32)
    }
}

impl From<LogLevel> for i32 {
    fn from(level: LogLevel) -> Self {
        level as i32
    }
}

impl From<LogLevel> for u32 {
    fn from(level: LogLevel) -> Self {
        level as u32
    }
}
