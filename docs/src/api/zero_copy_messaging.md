# eCAL Zero-Copy in Rustecal

This guide shows you how to publish and receive large binary payloads **without any memcpy** by using:

- **`TypedPublisher<BytesMessage>`** - a built-in byte-array message type  
- **`PayloadWriter`** - for direct in-place writes into eCAL's shared memory  

---

## 1. How eCAL Zero-Copy Works

eCAL zero-copy uses shared memory to avoid copying data between publisher and subscriber.  
- **Publisher** allocates a memory file and writes directly into it via `PayloadWriter`.  
- **Subscriber** maps the same memory file and reads the buffer in place.  
- Handshake and buffer management are handled by eCAL's SHM layer.

---

## 2. `PayloadWriter` API

A `PayloadWriter` lets you fill the shared-memory buffer in place:

```rust
pub trait PayloadWriter {
    /// Called once on first allocation or resize.
    fn write_full(&mut self, buf: &mut [u8]) -> bool;

    /// Called on subsequent sends to modify only parts of the buffer.
    fn write_modified(&mut self, buf: &mut [u8]) -> bool {
        self.write_full(buf)
    }

    /// Returns the exact number of bytes you will write.
    fn get_size(&self) -> usize;
}
````

Implement these methods for your payload type, then pass a mutable reference to `send_payload_writer`.

---

## 3. Publisher Sample

```rust
use rustecal::{Configuration, Ecal, EcalComponents, TypedPublisher};
use rustecal_pubsub::PayloadWriter;
use rustecal_pubsub::publisher::Timestamp;
use rustecal_types_bytes::BytesMessage;

/// A simple zero-copy writer that fills a buffer with a repeating pattern.
pub struct CustomWriter {
    size: usize,
    counter: u8,
}

impl CustomWriter {
    pub fn new(size: usize) -> Self {
        Self { size, counter: 0 }
    }
}

impl PayloadWriter for CustomWriter {
    fn write_full(&mut self, buf: &mut [u8]) -> bool {
        if buf.len() < self.size { return false; }
        // fill entire buffer with 0xAA
        buf[..self.size].fill(0xAA);
        true
    }

    fn write_modified(&mut self, buf: &mut [u8]) -> bool {
        if buf.len() < self.size { return false; }
        // flip one byte each time
        let idx = (self.counter as usize) % self.size;
        buf[idx] ^= 0xFF;
        self.counter = self.counter.wrapping_add(1);
        true
    }

    fn get_size(&self) -> usize {
        self.size
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // configure eCAL
    let mut cfg = Configuration::new()?;
    cfg.publisher.layer.shm.zero_copy_mode         = true as i32;
    cfg.publisher.layer.shm.acknowledge_timeout_ms = 50;
    Ecal::initialize(
        Some("zero copy publisher"),
        EcalComponents::DEFAULT,
        Some(&cfg),
    )?;

    // create typed publisher
    let publisher: TypedPublisher<BytesMessage> =
        TypedPublisher::new("buffer")?;

    // prepare zero-copy payload writer
    let mut writer = CustomWriter::new(8 * 1024 * 1024); // 8 MB

    // send loop
    while Ecal::ok() {
        publisher.send_payload_writer(&mut writer, Timestamp::Auto);
    }

    // finalize ecal and clean up
    Ecal::finalize();
    Ok(())
}
```

---

## 4. Subscriber Sample

```rust
use std::{thread, time::Duration};
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_bytes::BytesMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize eCAL
    Ecal::initialize(
        Some("zero copy subscriber"),
        EcalComponents::DEFAULT,
        None,
    )?;

    // create typed subscriber
    let mut sub: TypedSubscriber<BytesMessage> =
        TypedSubscriber::new("buffer")?;

    // register zero-copy callback
    sub.set_callback(|received| {
        // borrow shared-memory payload
        let buffer: &[u8] = received.payload.data.as_ref();
        // this line is just to demonstrate usage (it will kill the performance)
        println!("Received {} bytes", buffer.len());
    });

    // keep alive for callbacks
    while Ecal::ok() {
        thread::sleep(Duration::from_millis(100));
    }

    // finalize ecal and clean up
    Ecal::finalize();
    Ok(())
}
```
