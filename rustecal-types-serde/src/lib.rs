//! # rustecal-types-serde
//!
//! eCAL Pub/Sub support for Serde-enabled messages.

pub mod cbor_message;
pub mod format_support;
pub mod json_message;
pub mod msgpack_message;

pub use cbor_message::CborMessage;
pub use json_message::JsonMessage;
pub use msgpack_message::MsgpackMessage;
