//! A performance benchmark publisher in Rust, modeled on the eCAL C++ sample.
//!
//! This will send messages of the given size in a tight loop, logging
//! throughput every second.

use std::{env, sync::Arc, time::{Duration, Instant}};
use rustecal::{Ecal, EcalComponents, Configuration, TypedPublisher};
use rustecal_types_bytes::BytesMessage;

// performance settings
const ZERO_COPY:               bool  = true;
const BUFFER_COUNT:            u32   = 1;
const ACKNOWLEDGE_TIMEOUT_MS:  i32   = 50;
const PAYLOAD_SIZE_DEFAULT:    usize = 8 * 1024 * 1024;

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
    println!("Zero copy mode: {}", ZERO_COPY);
    println!("Number of write buffers: {}", BUFFER_COUNT);
    println!("Acknowledge timeout: {} ms", ACKNOWLEDGE_TIMEOUT_MS);
    println!("Payload size: {} bytes", payload_size);
    println!();

    // prepare and tweak eCAL Configuration
    let mut cfg = Configuration::new()?;
    cfg.publisher.layer.shm.zero_copy_mode         = ZERO_COPY as i32;
    cfg.publisher.layer.shm.memfile_buffer_count   = BUFFER_COUNT;
    cfg.publisher.layer.shm.acknowledge_timeout_ms = ACKNOWLEDGE_TIMEOUT_MS as u32;

    // initialize eCAL with custom config
    Ecal::initialize(
        Some("performance send rust"),
        EcalComponents::DEFAULT,
        Some(&cfg),
    )
    .expect("eCAL initialization failed");

    // create payload buffer and publisher
    let payload_vec: Vec<u8> = vec![0u8; payload_size];
    let mut payload: Arc<[u8]> = Arc::from(payload_vec);
    let publisher: TypedPublisher<BytesMessage> = TypedPublisher::new("Performance")?;

    // benchmark loop
    let mut msgs_sent  = 0u64;
    let mut bytes_sent = 0u64;
    let mut iterations = 0u64;
    let mut last_log   = Instant::now();

    // wait for at least one subscriber to be ready
    while publisher.get_subscriber_count() == 0 {
        println!("Waiting for performance receive to start ...");
        std::thread::sleep(Duration::from_millis(1000));
    }

    // send loop
    while Ecal::ok() {
        // modify the payload for each message
        {
            let buf: &mut [u8] = Arc::make_mut(&mut payload);
            let chr = (msgs_sent % 9 + 48) as u8;
            buf[0..16].fill(chr);
        }

        let wrapped = BytesMessage { data: payload.clone() };
        publisher.send(&wrapped);

        msgs_sent += 1;
        bytes_sent += payload_size as u64;
        iterations += 1;

        // every second, print statistics
        if iterations % 2000 == 0 {
            let elapsed = last_log.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let secs       = elapsed.as_secs_f64();
                let kbyte_s    = (bytes_sent as f64 / 1024.0) / secs;
                let mbyte_s    = kbyte_s / 1024.0;
                let gbyte_s    = mbyte_s / 1024.0;
                let msg_s      = (msgs_sent as f64) / secs;
                let latency_us = (secs * 1e6) / (msgs_sent as f64);

                println!("Payload size      : {} kB", payload_size / 1024);
                println!("Throughput (kB/s) : {:.0}", kbyte_s);
                println!("Throughput (MB/s) : {:.2}", mbyte_s);
                println!("Throughput (GB/s) : {:.3}", gbyte_s);
                println!("Messages/s        : {:.0}", msg_s);
                println!("Latency (µs)      : {:.2}", latency_us);
                println!();

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
