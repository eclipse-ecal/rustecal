use rustecal::{Ecal, EcalComponents};
use rustecal::{ServiceClient, ServiceRequest, CallState};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("mirror client rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let client = ServiceClient::new("mirror")?;

    // Wait until connected
    while client.get_client_instances().is_empty() {
        println!("Waiting for a service ..");
        thread::sleep(Duration::from_secs(1));
    }

    let methods = ["echo", "reverse"];
    let mut i = 0;

    while Ecal::ok() {
        let method_name = methods[i % methods.len()];
        i += 1;

        let request = ServiceRequest {
            payload: b"stressed".to_vec(),
        };

        for instance in client.get_client_instances() {
            let response = instance.call(method_name, request.clone(), Some(1000));

            println!();
            println!("Method '{}' called with message: stressed", method_name);

            match response {
                Some(res) => {
                    match CallState::from(res.success as i32) {
                        CallState::Executed => {
                            let text = String::from_utf8_lossy(&res.payload);
                            println!(
                                "Received response: {} from service id {:?}",
                                text, res.server_id.service_id.entity_id
                            );
                        }
                        CallState::Failed => {
                            println!(
                                "Received error: {} from service id {:?}",
                                res.error_msg.unwrap_or_else(|| "Unknown".into()),
                                res.server_id.service_id.entity_id
                            );
                        }
                        _ => {}
                    }
                }
                None => {
                    println!("Method blocking call failed ..");
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
