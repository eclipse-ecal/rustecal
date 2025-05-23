use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_serde::JsonMessage;
use rustecal::pubsub::typed_subscriber::Received;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct SimpleMessage {
    message: String,
    count: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("serde receive rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    // Create a typed subscriber for topic "simple_message"
    let mut subscriber: TypedSubscriber<JsonMessage<SimpleMessage>> =
        TypedSubscriber::new("simple_message")?;

    subscriber.set_callback(|msg: Received<JsonMessage<SimpleMessage>>| {
        println!("------------------------------------------");
        println!(" MESSAGE HEAD ");
        println!("------------------------------------------");
        println!("topic name   : {}", msg.topic_name);
        println!("encoding     : {}", msg.encoding);
        println!("type name    : {}", msg.type_name);
        println!("timestamp    : {}", msg.timestamp);
        println!("clock        : {}", msg.clock);
        println!("------------------------------------------");
        println!(" MESSAGE CONTENT ");
        println!("------------------------------------------");
        println!("message      : {}", msg.payload.data.message);
        println!("count        : {}", msg.payload.data.count);
        println!("------------------------------------------\n");
    });

    println!("Waiting for messages on topic 'simple_message'...");

    // keep the thread alive so callbacks can run
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
