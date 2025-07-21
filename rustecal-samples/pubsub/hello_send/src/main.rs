use rustecal::pubsub::publisher::Timestamp;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize eCAL
    Ecal::initialize(Some("hello send rust"), EcalComponents::DEFAULT, None)
        .expect("eCAL initialization failed");

    let publisher: TypedPublisher<StringMessage> = TypedPublisher::<StringMessage>::new("hello")?;

    let mut count = 0;
    while Ecal::ok() {
        count += 1;
        let msg = format!("HELLO WORLD FROM RUST ({count})");

        let wrapped = StringMessage { data: msg.into() };
        publisher.send(&wrapped, Timestamp::Auto);

        println!("Sent: {}", wrapped.data);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
