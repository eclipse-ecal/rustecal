use prost::Message;
use rustecal::{Ecal, EcalComponents};
use rustecal::{MethodInfo, ServiceServer};

// Add the protobuf compiled by prost
mod math_pb {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}
use math_pb::{SFloat, SFloatTuple};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize eCAL
    Ecal::initialize(Some("math server rust"), EcalComponents::DEFAULT, None)
        .expect("eCAL initialization failed");

    // create the service server named "MathService"
    let mut server = ServiceServer::new("MathService")?;

    // register "Add" method from protobuf rpc
    server.add_method(
        "Add",
        Box::new(|info: MethodInfo, req: &[u8]| {
            // Decode serialized protobuf request
            let req_pb = SFloatTuple::decode(req);

            match req_pb {
                Ok(pb) => {
                    println!(
                        "Received request for MathService in Rust: {}",
                        info.method_name
                    );
                    println!("Input1 : {}", pb.inp1);
                    println!("Input2 : {}", pb.inp2);
                    println!();

                    let result = SFloat {
                        out: pb.inp1 + pb.inp2,
                    };
                    // Serialize protobuf response
                    result.encode_to_vec()
                }
                _ => {
                    println!("Unable to decode protobuf");
                    vec![]
                }
            }
        }),
    )?;

    // register "Multiply" method from protobuf rpc
    server.add_method(
        "Multiply",
        Box::new(|info: MethodInfo, req: &[u8]| {
            // Decode serialized protobuf request
            let req_pb = SFloatTuple::decode(req);

            match req_pb {
                Ok(pb) => {
                    println!(
                        "Received request for MathService in Rust: {}",
                        info.method_name
                    );
                    println!("Input1 : {}", pb.inp1);
                    println!("Input2 : {}", pb.inp2);
                    println!();

                    let result = SFloat {
                        out: pb.inp1 * pb.inp2,
                    };
                    // Serialize protobuf response
                    result.encode_to_vec()
                }
                _ => {
                    println!("Unable to decode protobuf");
                    vec![]
                }
            }
        }),
    )?;

    // register "Divide" method from protobuf rpc
    server.add_method(
        "Divide",
        Box::new(|info: MethodInfo, req: &[u8]| {
            // Decode serialized protobuf request
            let req_pb = SFloatTuple::decode(req);

            match req_pb {
                Ok(pb) => {
                    println!(
                        "Received request for MathService in Rust: {}",
                        info.method_name
                    );
                    println!("Input1 : {}", pb.inp1);
                    println!("Input2 : {}", pb.inp2);
                    println!();

                    let result = SFloat {
                        out: pb.inp1 / pb.inp2,
                    };
                    // Serialize protobuf response
                    result.encode_to_vec()
                }
                _ => {
                    println!("Unable to decode protobuf");
                    vec![]
                }
            }
        }),
    )?;

    println!("Rust math service running. Press Ctrl+C to exit.");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
