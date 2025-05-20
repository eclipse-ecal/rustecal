use std::sync::Arc;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("hello send rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher: TypedPublisher<StringMessage> = TypedPublisher::<StringMessage>::new("hello")?;

    let mut cnt = 0;
    while Ecal::ok() {
        cnt += 1;
        let msg = format!("HELLO WORLD FROM RUST ({})", cnt);

        let wrapped = StringMessage{ data: Arc::<str>::from(msg) };
        publisher.send(&wrapped);

        println!("Sent: {}", wrapped.data);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
