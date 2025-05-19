use std::sync::Arc;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() {
    Ecal::initialize(Some("hello send rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher: TypedPublisher<StringMessage> = TypedPublisher::<StringMessage>::new("hello")
        .expect("Failed to create publisher");

    let mut cnt = 0;
    while Ecal::ok() {
        cnt += 1;
        let msg = format!("HELLO WORLD FROM RUST ({})", cnt);

        let wrapped = StringMessage(Arc::from(msg));
        publisher.send(&wrapped);

        println!("Sent: {}", wrapped.0);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}