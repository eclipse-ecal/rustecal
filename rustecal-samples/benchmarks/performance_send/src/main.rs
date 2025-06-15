//! A performance benchmark publisher in Rust, modeled on the eCAL C++ sample.
//!
//! This will send messages of the given size in a tight loop, logging
//! throughput every second.

use std::{env, time::{Duration, Instant}};
use rustecal::{Ecal, EcalComponents, Configuration, Publisher};
use rustecal_core::types::DataTypeInfo;
use std::thread::sleep;

mod binary_payload_writer;
use binary_payload_writer::BinaryPayload;

// performance settings
const ZERO_COPY:              bool  = true;
const BUFFER_COUNT:           u32   = 1;
const ACKNOWLEDGE_TIMEOUT_MS: i32   = 50;
const PAYLOAD_SIZE_DEFAULT:   usize = 8 * 1024 * 1024;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse payload size from CLI (or use default)
    let args: Vec<String> = env::args().collect();
    let mut payload_size = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(PAYLOAD_SIZE_DEFAULT)
    } else {
        PAYLOAD_SIZE_DEFAULT
    };
    if payload_size == 0 {
        payload_size = 1;
    }

    // log performance settings
    println!("Zero copy mode          : {}", ZERO_COPY);
    println!("Number of write buffers : {}", BUFFER_COUNT);
    println!("Acknowledge timeout     : {} ms", ACKNOWLEDGE_TIMEOUT_MS);
    println!("Payload size            : {} bytes", payload_size);
    println!();

    // configure eCAL
    let mut cfg = Configuration::new()?;
    cfg.publisher.layer.shm.zero_copy_mode         = ZERO_COPY as i32;
    cfg.publisher.layer.shm.memfile_buffer_count   = BUFFER_COUNT;
    cfg.publisher.layer.shm.acknowledge_timeout_ms = ACKNOWLEDGE_TIMEOUT_MS as u32;

    // initialize
    Ecal::initialize(
        Some("performance_send_rust"),
        EcalComponents::DEFAULT,
        Some(&cfg),
    )?;

    // create an untyped publisher
    let data_type_info = DataTypeInfo {
        encoding: "raw".into(),
        type_name: "bytes".into(),
        descriptor: Vec::new()
    };
    let publisher = Publisher::new("Performance", data_type_info)?;

    // prepare our zero-copy payload
    let mut payload = BinaryPayload::new(payload_size);

    // counters
    let mut msgs_sent  = 0u64;
    let mut bytes_sent = 0u64;
    let mut iterations = 0u64;
    let mut last_log   = Instant::now();

    // wait for subscriber
    while publisher.get_subscriber_count() == 0 {
        println!("Waiting for receiver …");
        sleep(Duration::from_secs(1));
    }
    println!();

    // send loop
    while Ecal::ok() {
        // send via zero-copy writer
        publisher.send_payload_writer(& mut payload, None);

        msgs_sent  += 1;
        bytes_sent += payload_size as u64;
        iterations += 1;

        // every ~2000 msgs, log if 1s has passed
        if iterations % 2000 == 0 {
            let elapsed = last_log.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let secs       = elapsed.as_secs_f64();
                let kbyte_s    = (bytes_sent as f64 / 1024.0) / secs;
                let mbyte_s    = kbyte_s / 1024.0;
                let gbyte_s    = mbyte_s / 1024.0;
                let msg_s      = (msgs_sent as f64) / secs;
                let latency_us = (secs * 1e6) / (msgs_sent as f64);

                println!("Payload size (kB)   : {:.0}", payload_size / 1024);
                println!("Throughput   (kB/s) : {:.0}", kbyte_s);
                println!("Throughput   (MB/s) : {:.2}", mbyte_s);
                println!("Throughput   (GB/s) : {:.2}", gbyte_s);
                println!("Messages     (1/s)  : {:.0}", msg_s);
                println!("Latency      (µs)   : {:.2}", latency_us);
                println!();

                // reset counters
                msgs_sent  = 0;
                bytes_sent = 0;
                last_log   = Instant::now();
            }
        }
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
