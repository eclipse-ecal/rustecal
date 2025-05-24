# rustecal-pubsub

`rustecal-pubsub` provides a high-level, type-safe Publisher/Subscriber API on top of eCAL’s raw FFI and Core API, enabling Rust applications to send and receive structured messages with minimal boilerplate.

## Features

- **Untyped Pub/Sub** for raw buffers when needed
- **Typed Pub/Sub** via `TypedPublisher<T>` and `TypedSubscriber<T>`
- **Support for arbitrary message types** implementing the `PublisherMessage` and `SubscriberMessage` traits
- **Metadata propagation**: topics carry encoding, type name, and optional descriptor

## Requirements

- **Rust** 1.60 or later  
- **Eclipse eCAL** C/C++ library v6.0 or later installed and accessible on your system

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rustecal-pubsub = "0.1"
```

## Quickstart

### Typed Publisher Example

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

### Typed Subscriber Example

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

- `PublisherMessage`: Defines `datatype()` and `to_bytes()` for a message type.
- `SubscriberMessage`: Defines `datatype()` and `from_bytes()` for reconstructing a message.

Implement these traits to integrate custom types or leverage helper crates like `rustecal-types-protobuf` or `rustecal-types-serde`.

## Advanced Usage

- Untyped Pub/Sub: Use `rustecal_pubsub::Publisher` and `Subscriber` for raw buffers.
- Metadata Inspection: Retrieve topic metadata via `get_data_type_information()`.
- Message-format support: Combine with `rustecal-types-bytes`, `rustecal-types-string`, `rustecal-types-protobuf` for Bytes, String, and Protobuf.
- Message-format support: Combine with `rustecal-types-serde` for JSON, CBOR, and MessagePack.

For more examples, see the `rustecal-samples/pubsub`.