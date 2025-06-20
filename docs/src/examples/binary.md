# Binary Message Example

## Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal::pubsub::publisher::Timestamp;
use rustecal_types_bytes::BytesMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("blob publisher"), EcalComponents::DEFAULT, None)?;

    let publisher = TypedPublisher::<BytesMessage>::new("blob")?;

    let mut counter = 0u8;
    while Ecal::ok() {
        let buf = vec![counter; 1024];
        counter = counter.wrapping_add(1);

        let message = BytesMessage { data: buf.into() };
        publisher.send(&message, Timestamp::Auto);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```

## Subscriber

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_bytes::BytesMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("blob subscriber"), EcalComponents::DEFAULT, None)?;

    let mut subscriber = TypedSubscriber::<BytesMessage>::new("blob")?;
    subscriber.set_callback(|message| {
        println!("Received blob of {} bytes", message.payload.data.len());
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```
