use rustecal::{Ecal, EcalComponents};
use rustecal_core::monitoring::Monitoring;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL (only the monitoring component)
    Ecal::initialize(Some("monitoring receive sample"), EcalComponents::MONITORING)?;
    println!("eCAL initialized. Entering monitoring loop…");

    while Ecal::ok() {
        let snap = Monitoring::get_snapshot()?;

        println!("=== Monitoring Snapshot ===\n");

        println!("Processes:\n{:#?}", snap.processes);
        println!("\nPublishers:\n{:#?}", snap.publishers);
        println!("\nSubscribers:\n{:#?}", snap.subscribers);
        println!("\nServers:\n{:#?}", snap.servers);
        println!("\nClients:\n{:#?}", snap.clients);

        // Sleep before next poll
        thread::sleep(Duration::from_secs(1));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
