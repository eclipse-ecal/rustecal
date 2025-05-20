use rustecal::{Ecal, EcalComponents};
use rustecal::{ServiceServer, MethodInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("mirror server rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create the service server named "mirror"
    let mut server = ServiceServer::new("mirror")?;

    // Register "echo" method: respond with request unchanged
    server.add_method("echo", Box::new(|info: MethodInfo, req: &[u8]| {
        println!("Method   : '{}' called", info.method_name);
        println!("Request  : {}", String::from_utf8_lossy(req));
        println!("Response : {}\n", String::from_utf8_lossy(req));
        req.to_vec()
    }))?;

    // Register "reverse" method: respond with request reversed
    server.add_method("reverse", Box::new(|info: MethodInfo, req: &[u8]| {
        let mut reversed = req.to_vec();
        reversed.reverse();
        println!("Method   : '{}' called", info.method_name);
        println!("Request  : {}", String::from_utf8_lossy(req));
        println!("Response : {}\n", String::from_utf8_lossy(&reversed));
        reversed
    }))?;

    println!("Rust mirror service running. Press Ctrl+C to exit.");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
