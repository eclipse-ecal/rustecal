# String Message Example

## Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("string publisher"), EcalComponents::DEFAULT, None)?;

    let publisher = TypedPublisher::<StringMessage>::new("hello")?;

    while Ecal::ok() {
        let message = StringMessage { data: "Hello from Rust".into() };
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
use rustecal_types_string::StringMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("string subscriber"), EcalComponents::DEFAULT, None)?;

    let mut subscriber = TypedSubscriber::<StringMessage>::new("hello")?;
    subscriber.set_callback(|message| {
        println!("Received: {}", message.payload.data)
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```
