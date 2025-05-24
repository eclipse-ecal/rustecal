# rustecal-types-string

`rustecal-types-string` provides a simple wrapper for UTF-8 string messages (`Arc<str>`) to use with the typed eCAL Pub/Sub API.

## Features

- **StringMessage**: wrap and transport UTF-8 string payloads
- Implements `PublisherMessage` and `SubscriberMessage` for seamless integration
- Zero-copy where possible via `Arc<str>`
- No extra dependencies beyond `rustecal-core` and `rustecal-pubsub`

## Installation

Add to your **workspace** `Cargo.toml`:

```toml
[dependencies]
rustecal-types-string = "0.1"
```

## Usage

### Publisher Example

```rust
use std::sync::Arc;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("string publisher"), EcalComponents::DEFAULT)?;

    let publisher = TypedPublisher::<StringMessage>::new("hello")?;

    while Ecal::ok() {
        let message = StringMessage { data: Arc::from("Hello from Rust") };
        publisher.send(&message);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```

### Subscriber Example

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("string subscriber"), EcalComponents::DEFAULT)?;

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

## Traits Reference

- **`PublisherMessage`**

  - `fn datatype() -> DataTypeInfo`
  - `fn to_bytes(&self) -> Arc<[u8]>`

- **`SubscriberMessage`**

  - `fn datatype() -> DataTypeInfo`
  - `fn from_bytes(bytes: Arc<[u8]>, _data_type_info: &DataTypeInfo) -> Option<Self>`

## See Also

- `rustecal-types-bytes` for raw binary data messages
- `rustecal-types-protobuf` for Protobuf-based messages
- `rustecal-types-serde` for JSON/CBOR/MessagePack via Serde
- Examples in the `rustecal-samples/pubsub` directory
