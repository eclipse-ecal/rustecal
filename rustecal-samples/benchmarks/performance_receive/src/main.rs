//! A performance benchmark subscriber in Rust, modeled on the eCAL C++ sample.
//!

use std::{sync::{Arc, Mutex, atomic::Ordering}, thread, time::{Duration, Instant}};
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal::pubsub::typed_subscriber::Received;
use rustecal_types_bytes::BytesMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize eCAL
    Ecal::initialize(Some("performance receive rust"), EcalComponents::DEFAULT, None)?;

    // create a typed subscriber for raw bytes
    let mut subscriber: TypedSubscriber<BytesMessage> = TypedSubscriber::new("Performance")?;

    // shared counters & timer
    let msgs  = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let bytes = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let start = Arc::new(Mutex::new(Instant::now()));

    // register the receive‐callback
    {
        let msgs  = Arc::clone(&msgs);
        let bytes = Arc::clone(&bytes);
        let start = Arc::clone(&start);

        subscriber.set_callback(move |msg: Received<BytesMessage>| {
            let buffer = &msg.payload.data;
            if buffer.is_empty() {
                // nothing to do
                return;
            }

            // update counters
            msgs.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            bytes.fetch_add(buffer.len() as u64, std::sync::atomic::Ordering::Relaxed);

            // lock the timer, compute & maybe print
            let mut start_lock = start.lock().unwrap();
            let elapsed = start_lock.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let m = msgs.swap(0, Ordering::Relaxed);
                let b = bytes.swap(0, Ordering::Relaxed);

                let secs = elapsed.as_secs_f64();
                let kbyte_s    = (b as f64 / 1024.0) / secs;
                let mbyte_s    = kbyte_s / 1024.0;
                let gbyte_s    = mbyte_s / 1024.0;
                let msg_s      = (m as f64) / secs;
                let latency_us = (secs * 1e6) / (m as f64);

                println!("Payload size      : {} kB", buffer.len() / 1024);
                println!("Throughput (kB/s) : {:.0}", kbyte_s);
                println!("Throughput (MB/s) : {:.2}", mbyte_s);
                println!("Throughput (GB/s) : {:.3}", gbyte_s);
                println!("Messages/s        : {:.0}", msg_s);
                println!("Latency (µs)      : {:.2}", latency_us);
                println!();

                // reset the timer
                *start_lock = Instant::now();
            }
        });
    }

    // keep the thread alive so callbacks can run
    while Ecal::ok() {
        thread::sleep(Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
