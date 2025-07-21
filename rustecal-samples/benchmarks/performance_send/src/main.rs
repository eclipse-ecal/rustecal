//! A performance benchmark publisher in Rust, using the typed `BytesMessage` publisher
//! to demonstrate zero-copy payload support.
//!
//! Sends messages of the given size in a tight loop, logging throughput every second.

use rustecal::{Configuration, Ecal, EcalComponents, TypedPublisher};
use rustecal_types_bytes::BytesMessage;
use std::thread::sleep;
use std::{
    env,
    time::{Duration, Instant},
};

mod binary_payload_writer;
use binary_payload_writer::BinaryPayload;
use rustecal_pubsub::publisher::Timestamp;

// performance settings
const ZERO_COPY: bool = true;
const BUFFER_COUNT: u32 = 1;
const ACKNOWLEDGE_TIMEOUT_MS: i32 = 50;
const PAYLOAD_SIZE_DEFAULT: usize = 8 * 1024 * 1024;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse payload size from CLI (or use default)
    let payload_size = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(PAYLOAD_SIZE_DEFAULT);

    // log performance settings
    println!("Zero copy mode          : {ZERO_COPY}");
    println!("Number of write buffers : {BUFFER_COUNT}");
    println!("Acknowledge timeout     : {ACKNOWLEDGE_TIMEOUT_MS} ms");
    println!("Payload size            : {payload_size} bytes");
    println!();

    // configure eCAL
    let mut cfg = Configuration::new()?;
    cfg.publisher.layer.shm.zero_copy_mode = ZERO_COPY as i32;
    cfg.publisher.layer.shm.memfile_buffer_count = BUFFER_COUNT;
    cfg.publisher.layer.shm.acknowledge_timeout_ms = ACKNOWLEDGE_TIMEOUT_MS as u32;

    // initialize eCAL
    Ecal::initialize(
        Some("performance send rust"),
        EcalComponents::DEFAULT,
        Some(&cfg),
    )?;

    // create a typed publisher for raw bytes
    let publisher: TypedPublisher<BytesMessage> = TypedPublisher::new("Performance")?;

    // prepare our zero-copy payload writer
    let mut payload = BinaryPayload::new(payload_size);

    // counters and timer
    let mut msgs_sent = 0u64;
    let mut bytes_sent = 0u64;
    let mut iterations = 0u64;
    let mut last_log = Instant::now();

    // wait for subscriber
    while publisher.get_subscriber_count() == 0 {
        println!("Waiting for receiver …");
        sleep(Duration::from_secs(1));
    }
    println!();

    // send loop
    while Ecal::ok() {
        // zero-copy send via PayloadWriter
        publisher.send_payload_writer(&mut payload, Timestamp::Auto);

        msgs_sent += 1;
        bytes_sent += payload_size as u64;
        iterations += 1;

        // every ~2000 msgs, log if 1s has passed
        if iterations % 2000 == 0 && last_log.elapsed() >= Duration::from_secs(1) {
            let secs = last_log.elapsed().as_secs_f64();
            let kbyte_s = (bytes_sent as f64 / 1024.0) / secs;
            let mbyte_s = kbyte_s / 1024.0;
            let gbyte_s = mbyte_s / 1024.0;
            let msg_s = (msgs_sent as f64) / secs;
            let latency_us = (secs * 1e6) / (msgs_sent as f64);

            println!("Payload size (kB)   : {}", payload_size / 1024);
            println!("Throughput (kB/s)   : {kbyte_s:.0}");
            println!("Throughput (MB/s)   : {mbyte_s:.2}");
            println!("Throughput (GB/s)   : {gbyte_s:.2}");
            println!("Messages     (1/s)  : {msg_s:.0}");
            println!("Latency      (µs)   : {latency_us:.2}");
            println!();

            // reset counters and timer
            msgs_sent = 0;
            bytes_sent = 0;
            last_log = Instant::now();
        }
    }

    // finalize eCAL
    Ecal::finalize();
    Ok(())
}
