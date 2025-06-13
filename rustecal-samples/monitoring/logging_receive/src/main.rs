use rustecal::{Ecal, EcalComponents};
use rustecal_core::log::Log;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize eCAL (only the logging component)
    Ecal::initialize(Some("logging receive sample"), EcalComponents::LOGGING, None)?;
    println!("eCAL initialized. Entering logging loop…");

    while Ecal::ok() {
        // fetch whatever log entries are available
        let entries = Log::get_logging()?;

        println!("=== Logging Snapshot ===\n");
        println!("Entries:\n{:#?}", entries);

        // sleep before next poll
        thread::sleep(Duration::from_secs(1));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
