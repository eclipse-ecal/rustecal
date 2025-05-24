# rustecal-types-serde

`rustecal-types-serde` provides Serde-based message wrappers for JSON, CBOR, and MessagePack to use with the typed eCAL Pub/Sub API.

## Features

- **JsonMessage<T>**, **CborMessage<T>**, **MsgpackMessage<T>**: per-format wrappers
- Implements `PublisherMessage` and `SubscriberMessage` for seamless integration
- Zero-copy payloads via `Arc<T>`
- Minimal dependencies: `serde`, `serde_json`, `serde_cbor`, `rmp-serde`, `rustecal-core`, `rustecal-pubsub`

## Installation

Add to your **workspace** `Cargo.toml`:

```toml
[dependencies]
rustecal-types-serde = "0.1"
```

## Usage

### Publisher Example (JSON)

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

### Subscriber Example (JSON)

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

## Traits Reference

- **`PublisherMessage`**
  - `datatype() -> DataTypeInfo`
  - `to_bytes(&self) -> Arc<[u8]>`

- **`SubscriberMessage`**
  - `datatype() -> DataTypeInfo`
  - `from_bytes(bytes: Arc<[u8]>, _info: &DataTypeInfo) -> Option<Self>`

## See Also

- `rustecal-types-bytes` for raw binary data messages
- `rustecal-types-protobuf` for Protobuf-based messages
- `rustecal-types-string` for UTF-8 string messages
- Examples in the `rustecal-samples/pubsub` directory
