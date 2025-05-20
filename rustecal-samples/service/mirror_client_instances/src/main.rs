use rustecal::{Ecal, EcalComponents};
use rustecal::{ServiceClient, ServiceRequest};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("mirror client instances rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let client = ServiceClient::new("mirror")?;
    let methods = ["echo", "reverse"];
    let mut i = 0;

    while Ecal::ok() {
        let method = methods[i % methods.len()];
        i += 1;

        let request = ServiceRequest {
            payload: b"stressed".to_vec(),
        };

        let instances = client.get_client_instances();

        if instances.is_empty() {
            println!("No service instances available.");
        } else {
            for instance in instances {
                let response = instance.call(method, request.clone(), Some(1000));

                println!();
                println!(
                    "Method '{}' called with message: stressed",
                    method
                );

                match response {
                    Some(res) => {
                        if res.success {
                            println!(
                                "Received response: {} from service id {:?}",
                                String::from_utf8_lossy(&res.payload),
                                res.server_id.service_id.entity_id
                            );
                        } else {
                            println!(
                                "Received error: {} from service id {:?}",
                                res.error_msg.unwrap_or_else(|| "Unknown".into()),
                                res.server_id.service_id.entity_id
                            );
                        }
                    }
                    None => {
                        println!("Call failed or timed out.");
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
