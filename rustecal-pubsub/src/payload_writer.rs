// payload_writer.rs
//
// Defines the `PayloadWriter` trait for zero-copy shared-memory payload writes,
// plus thread-local storage and C-style callback functions to integrate
// with eCAL's `SendPayloadWriter` API, using mutable references rather than owning values.

use std::cell::RefCell;
use std::os::raw::{c_int, c_void};

/// A zero‐copy payload writer: you fill the shared‐memory buffer in place.
pub trait PayloadWriter {
    /// Called once when the memory is first allocated (or resized).
    /// Return `true` on success.
    fn write_full(&mut self, buf: &mut [u8]) -> bool;

    /// Called on subsequent sends to modify only parts of the buffer.
    /// By default this falls back to `write_full`.
    fn write_modified(&mut self, buf: &mut [u8]) -> bool {
        self.write_full(buf)
    }

    /// Must return the exact number of bytes you’ll write.
    fn get_size(&self) -> usize;
}

// Thread-local slot for the currently active writer reference during a send call
thread_local! {
    /// Holds a raw pointer to the active PayloadWriter while eCAL invokes callbacks
    pub(crate) static CURRENT_WRITER: RefCell<Option<*mut dyn PayloadWriter>> = RefCell::new(None);
}

/// C callback: perform a full write into the shared-memory buffer
pub(crate) unsafe extern "C" fn write_full_cb(buffer: *mut c_void, size: usize) -> c_int {
    CURRENT_WRITER.with(|cell| {
        if let Some(writer_ptr) = *cell.borrow() {
            let writer: &mut dyn PayloadWriter = &mut *writer_ptr;
            let buf = std::slice::from_raw_parts_mut(buffer as *mut u8, size);
            if writer.write_full(buf) {
                0
            } else {
                -1
            }
        } else {
            -1
        }
    })
}

/// C callback: perform a partial modification of the shared-memory buffer
pub(crate) unsafe extern "C" fn write_mod_cb(buffer: *mut c_void, size: usize) -> c_int {
    CURRENT_WRITER.with(|cell| {
        if let Some(writer_ptr) = *cell.borrow() {
            let writer: &mut dyn PayloadWriter = &mut *writer_ptr;
            let buf = std::slice::from_raw_parts_mut(buffer as *mut u8, size);
            if writer.write_modified(buf) {
                0
            } else {
                -1
            }
        } else {
            -1
        }
    })
}

/// C callback: return the size of the payload buffer needed
pub(crate) unsafe extern "C" fn get_size_cb() -> usize {
    CURRENT_WRITER.with(|cell| {
        if let Some(writer_ptr) = *cell.borrow() {
            let writer: &mut dyn PayloadWriter = &mut *writer_ptr;
            writer.get_size()
        } else {
            0
        }
    })
}
