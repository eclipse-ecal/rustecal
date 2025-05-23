use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_serde::JsonMessage;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct SimpleMessage {
    message: String,
    count: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("serde send rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create a typed publisher for topic "simple_message"
    let publisher: TypedPublisher<JsonMessage<SimpleMessage>> =
        TypedPublisher::new("simple_message")?;

    let mut count = 0u64;
    while Ecal::ok() {
        count += 1;
        let payload = SimpleMessage {
            count,
            message: "HELLO WORLD FROM RUST".into(),
        };
        let wrapped = JsonMessage::new(payload.clone());

        // Send over eCAL pub/sub
        publisher.send(&wrapped);
        println!(
            "Sent: message = {}, count = {}",
            wrapped.data.message, wrapped.data.count
        );

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // Clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
