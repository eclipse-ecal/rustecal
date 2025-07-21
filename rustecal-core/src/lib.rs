//! # rustecal-core
//!
//! Provides core functionality for managing the eCAL runtime environment.
//!
//! Includes:
//! - Initialization (`Ecal::initialize`)
//! - Finalization (`Ecal::finalize`)
//! - System status queries and component management.
//!
//! This crate is typically re-exported by the `rustecal` crate.

pub mod components;
pub mod configuration;
pub mod core;
pub mod core_types;
pub mod error;
pub mod log;
pub mod log_level;
pub mod monitoring;
pub mod types;

// Re‑exports for ergonomic access:
pub use components::EcalComponents;
pub use configuration::Configuration;
pub use core::Ecal;
pub use core_types::logging::LogMessage;
pub use error::RustecalError;
pub use log::Log;
pub use log_level::LogLevel;
