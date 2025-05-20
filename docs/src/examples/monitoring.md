# Monitoring Snapshot

This example demonstrates how to continuously poll the eCAL runtime for a **monitoring snapshot** using the `Monitoring::get_snapshot()` API.

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal_core::monitoring::Monitoring;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize only the monitoring subsystem
    Ecal::initialize(Some("monitoring_receive_sample"), EcalComponents::MONITORING)?;

    while Ecal::ok() {
        let snap = Monitoring::get_snapshot()?;

        println!("=== Monitoring Snapshot ===\n");
        println!("Processes:\n{:#?}", snap.processes);
        println!("\nPublishers:\n{:#?}", snap.publishers);
        println!("\nSubscribers:\n{:#?}", snap.subscribers);
        println!("\nServers:\n{:#?}", snap.servers);
        println!("\nClients:\n{:#?}", snap.clients);

        thread::sleep(Duration::from_secs(1));
    }

    Ecal::finalize();
    Ok(())
}
```
