//! Component selection for eCAL initialization.
//!
//! This module provides the [`EcalComponents`] bitflag struct used to specify
//! which parts of the eCAL middleware should be activated during initialization.
//!
//! The flags can be combined using bitwise OR operations (e.g., `PUBLISHER | LOGGING`).
//!
//! These flags are passed to [`crate::Ecal::initialize`] to enable
//! or disable subsystems for performance, resource usage, or system design reasons.

use bitflags::bitflags;

bitflags! {
    /// Bitflags representing the subsystems of eCAL that can be individually enabled.
    ///
    /// These flags are used with [`Ecal::initialize`](crate::ecal::core::Ecal::initialize)
    /// to control which components are active.
    #[derive(Default)]
    pub struct EcalComponents: u32 {
        /// Disable all components (no subsystems enabled).
        const NONE       = 0x000;

        /// Enable the publish interface.
        const PUBLISHER  = 0x001;

        /// Enable the subscribe interface.
        const SUBSCRIBER = 0x002;

        /// Enable eCAL service (RPC-style communication).
        const SERVICE    = 0x004;

        /// Enable the monitoring component (e.g., for visualizing in eCAL Monitor).
        const MONITORING = 0x008;

        /// Enable logging (console/file output from eCAL runtime).
        const LOGGING    = 0x010;

        /// Enable time synchronization (e.g., simulated time or global time sync).
        const TIMESYNC   = 0x020;

        /// Common default configuration used for most applications.
        ///
        /// This enables all major communication components:
        /// - `PUBLISHER`
        /// - `SUBSCRIBER`
        /// - `SERVICE`
        /// - `LOGGING`
        /// - `TIMESYNC`
        ///
        /// It does **not** include `MONITORING`, which can be enabled separately.
        const DEFAULT = Self::PUBLISHER.bits()
                      | Self::SUBSCRIBER.bits()
                      | Self::SERVICE.bits()
                      | Self::LOGGING.bits()
                      | Self::TIMESYNC.bits();

        /// Enables all available components.
        ///
        /// This is equivalent to enabling every individual flag.
        const ALL     = Self::PUBLISHER.bits()
                      | Self::SUBSCRIBER.bits()
                      | Self::SERVICE.bits()
                      | Self::MONITORING.bits()
                      | Self::LOGGING.bits()
                      | Self::TIMESYNC.bits();
    }
}
