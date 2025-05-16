# String Message Example

## Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("string publisher"), EcalComponents::DEFAULT)?;

    let publisher = TypedPublisher::<StringMessage>::new("hello")?;

    while Ecal::ok() {
        let msg = StringMessage(format!("Hello from Rust"));
        publisher.send(&msg);

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
    Ecal::initialize(Some("string subscriber"), EcalComponents::DEFAULT)?;

    let mut sub = TypedSubscriber::<StringMessage>::new("hello")?;
    sub.set_callback(|msg| println!("Received: {}", msg.msg.0));

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```
