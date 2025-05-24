//! # rustecal-sys
//!
//! Raw FFI bindings to the eCAL C API.
//!
//! This crate is not intended for direct use, but underpins the safe abstractions
//! provided in `rustecal-core`, `rustecal-pubsub`, and `rustecal-service`.

// src/lib.rs
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![cfg_attr(docsrs, doc(hidden))]

#[cfg(not(docsrs))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// stub out on docs.rs so include! never fails
#[cfg(docsrs)]
mod bindings {}
