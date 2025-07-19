use rustecal::pubsub::typed_subscriber::Received;
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize eCAL
    Ecal::initialize(Some("hello receive rust"), EcalComponents::DEFAULT, None)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<StringMessage>::new("hello")?;

    subscriber.set_callback(|msg: Received<StringMessage>| {
        println!("------------------------------------------");
        println!(" MESSAGE HEAD ");
        println!("------------------------------------------");
        println!("topic name   : {}", msg.topic_name);
        println!("encoding     : {}", msg.encoding);
        println!("type name    : {}", msg.type_name);
        println!("topic time   : {}", msg.timestamp);
        println!("topic clock  : {}", msg.clock);
        println!("------------------------------------------");
        println!(" MESSAGE CONTENT ");
        println!("------------------------------------------");
        println!("message      : {}", msg.payload.data);
        println!("------------------------------------------\n");
    });

    println!("Waiting for messages on topic 'hello'...");

    // keep the thread alive so callbacks can run
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
