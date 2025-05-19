use std::sync::Arc;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_bytes::BytesMessage;

fn main() {
    Ecal::initialize(Some("blob send rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher = TypedPublisher::<BytesMessage>::new("blob")
        .expect("Failed to create publisher");

    let mut counter: u8 = 0;

    while Ecal::ok() {
        // Fill 1024-byte buffer with the current counter value
        let buffer = vec![counter; 1024];
        counter = counter.wrapping_add(1);

        let wrapped = BytesMessage(Arc::from(buffer));
        publisher.send(&wrapped);

        println!("Sent buffer filled with {}", counter);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
