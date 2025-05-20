# Logging Snapshot

This example demonstrates how to continuously poll the eCAL runtime for **log messages** using the `Log::get_logging()` API.

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal_core::log::Log;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize only the logging subsystem
    Ecal::initialize(Some("logging_receive_sample"), EcalComponents::LOGGING)?;

    while Ecal::ok() {
        let entries = Log::get_logging()?;
        println!("=== Logging Snapshot ===\n");
        println!("Entries:\n{:#?}", entries);

        thread::sleep(Duration::from_secs(1));
    }

    Ecal::finalize();
    Ok(())
}
```
