# JSON Message Example

## Publisher

```rust
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_serde::JsonMessage;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MyData {
    msg: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("json publisher"), EcalComponents::DEFAULT)?;

    let publisher = TypedPublisher::<JsonMessage<MyData>>::new("hello_json")?;

    while Ecal::ok() {
        let payload = MyData { msg: "Hello from Rust".into() };
        let message = JsonMessage::new(payload);
        publisher.send(&message);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```

## Subscriber

```rust
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_serde::JsonMessage;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MyData {
    msg: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("json subscriber"), EcalComponents::DEFAULT)?;

    let mut subscriber = TypedSubscriber::<JsonMessage<MyData>>::new("hello_json")?;
    subscriber.set_callback(|message| {
        println!("Received: {}", message.payload.data.msg);
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```
