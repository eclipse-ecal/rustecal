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

pub mod core;
pub mod components;
pub mod configuration;
pub mod error;
pub mod types;
pub mod log;
pub mod log_level;
pub mod core_types;
pub mod monitoring;

// Re‑exports for ergonomic access:
pub use core::Ecal;
pub use components::EcalComponents;
pub use configuration::Configuration;
pub use error::RustecalError;
pub use log::Log;
pub use log_level::LogLevel;
pub use core_types::logging::LogMessage;
