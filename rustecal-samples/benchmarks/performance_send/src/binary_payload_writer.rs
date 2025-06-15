//! Defines a simple `BinaryPayload` writer for zero-copy benchmarks.
//!
//! This payload writer mirrors the C++ `CBinaryPayload` example:
//! - `write_full` fills the entire buffer with a constant byte (0x2A).
//! - `write_modified` updates a single byte per invocation to simulate changing data.
//! - `get_size` reports the exact buffer size needed.

use rustecal_pubsub::PayloadWriter;

/// A direct-write binary payload that writes into shared memory without copying.
pub struct BinaryPayload {
    /// Total size, in bytes, of the payload to allocate and write.
    size: usize,
    /// A simple counter used to vary which byte is modified each call.
    clock: u32,
}

impl BinaryPayload {
    /// Create a new `BinaryPayload` of the given size.
    ///
    /// # Arguments
    ///
    /// * `size` – The number of bytes to allocate and write into.
    ///
    /// # Returns
    ///
    /// A fresh `BinaryPayload` instance with its internal clock reset.
    pub fn new(size: usize) -> Self {
        BinaryPayload { size, clock: 0 }
    }
}

impl PayloadWriter for BinaryPayload {
    /// Fill the entire buffer with the constant byte `0x2A`.
    ///
    /// This is called by eCAL when the shared-memory region is first allocated
    /// or its size changes. Returning `false` indicates an allocation error.
    fn write_full(&mut self, buf: &mut [u8]) -> bool {
        if buf.len() < self.size {
            // Buffer too small: cannot satisfy payload size
            return false;
        }
        // Fast-path fill: every byte set to 42 (0x2A)
        buf[..self.size].fill(42);
        true
    }

    /// Modify a single byte in the existing buffer to simulate an update.
    ///
    /// This is called after the first full write when zero-copy mode is enabled.
    /// It only changes one byte per call for maximum performance.
    fn write_modified(&mut self, buf: &mut [u8]) -> bool {
        if buf.len() < self.size {
            // Buffer too small: cannot satisfy payload size
            return false;
        }
        // Compute an index that cycles through the first 1024 bytes, then wraps
        let idx = ((self.clock as usize) % 1024) % self.size;
        // Overwrite that byte with ASCII digit '0'..'9'
        buf[idx] = b'0' + (self.clock % 10) as u8;
        // Advance the clock for next iteration
        self.clock = self.clock.wrapping_add(1);
        true
    }

    /// Report the exact payload size that this writer will produce.
    ///
    /// eCAL uses this to allocate the shared-memory region.
    fn get_size(&self) -> usize {
        self.size
    }
}
