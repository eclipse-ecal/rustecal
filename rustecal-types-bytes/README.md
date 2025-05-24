# rustecal-types-bytes

`rustecal-types-bytes` provides a simple wrapper for arbitrary binary data (`Arc<[u8]>`) to use with the typed eCAL Pub/Sub API.

## Features

- **BytesMessage**: wrap and transport raw binary payloads
- Implements `PublisherMessage` and `SubscriberMessage` for seamless integration
- Zero-copy where possible via `Arc<[u8]>`
- No extra dependencies beyond `rustecal-core` and `rustecal-pubsub`

## Installation

Add to your **workspace** `Cargo.toml`:

```toml
[dependencies]
rustecal-types-bytes = "0.1"
```

## Usage

### Publisher Example

```rust
use std::sync::Arc;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_bytes::BytesMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("blob publisher"), EcalComponents::DEFAULT)?;

    let publisher = TypedPublisher::<BytesMessage>::new("blob")?;

    let mut counter = 0u8;
    while Ecal::ok() {
        let buf = vec![counter; 1024];
        counter = counter.wrapping_add(1);

        let message = BytesMessage { data: Arc::from(buf) };
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
use rustecal_types_bytes::BytesMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("blob subscriber"), EcalComponents::DEFAULT)?;

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

## Traits Reference

- **`PublisherMessage`**
  - `datatype() -> DataTypeInfo`
  - `to_bytes(&self) -> Arc<[u8]>`

- **`SubscriberMessage`**
  - `datatype() -> DataTypeInfo`
  - `from_bytes(bytes: Arc<[u8]>, _info: &DataTypeInfo) -> Option<Self>`

## See Also

- `rustecal-types-string` for UTF-8 string messages
- `rustecal-types-protobuf` for Protobuf-based messages
- `rustecal-types-serde` for JSON/CBOR/MessagePack via Serde
- Examples in the `rustecal-samples/pubsub` directory
