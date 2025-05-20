use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal::pubsub::typed_subscriber::Received;
use rustecal_types_bytes::BytesMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("blob receive rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<BytesMessage>::new("blob")?;

    subscriber.set_callback(|msg: Received<BytesMessage>| {
        let buffer = &msg.payload.data;
        if buffer.is_empty() {
            return;
        }

        let content = buffer[0] as u8;

        println!("------------------------------------------");
        println!(" HEAD ");
        println!("------------------------------------------");
        println!("topic name   : {}", msg.topic_name);
        println!("encoding     : {}", msg.encoding);
        println!("type name    : {}", msg.type_name);
        println!("topic time   : {}", msg.timestamp);
        println!("topic clock  : {}", msg.clock);
        println!("------------------------------------------");
        println!(" CONTENT ");
        println!("------------------------------------------");
        println!("binary value : {}", content);
        println!("buffer size  : {}", buffer.len());
        println!("------------------------------------------\n");
    });

    println!("Waiting for messages on topic 'blob'...");

    // keep the thread alive so callbacks can run
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
