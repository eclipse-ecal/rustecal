use prost::Message;
use rustecal::{CallState, ServiceClient, ServiceRequest};
use rustecal::{Ecal, EcalComponents};
use std::thread;
use std::time::Duration;

// Add the protobuf compiled by prost
mod math_pb {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}
use math_pb::{SFloat, SFloatTuple};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize eCAL
    Ecal::initialize(Some("math client rust"), EcalComponents::DEFAULT, None)
        .expect("eCAL initialization failed");

    let client = ServiceClient::new("MathService")?;

    // wait until connected
    while client.get_client_instances().is_empty() {
        println!("Waiting for a service ..");
        thread::sleep(Duration::from_secs(1));
    }

    let methods = ["Add", "Multiply", "Divide"];
    let mut i = 0;

    while Ecal::ok() {
        let method_name = methods[i % methods.len()];
        i += 1;

        let payload_pb = SFloatTuple {
            inp1: i as f64,
            inp2: (i + 1) as f64,
        };

        let request = ServiceRequest {
            payload: payload_pb.encode_to_vec(),
        };

        for instance in client.get_client_instances() {
            let response = instance.call(method_name, request.clone(), Some(1000));

            println!();
            println!(
                "Method '{}' called with message: {:?}",
                method_name, payload_pb
            );

            match response {
                Some(res) => match CallState::from(res.success as i32) {
                    CallState::Executed => {
                        let response_data = SFloat::decode(&res.payload[..])?;
                        println!(
                            "Received response: {:?} from service id {:?}",
                            response_data, res.server_id.service_id.entity_id
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
                },
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
