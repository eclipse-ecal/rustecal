# rustecal-types-protobuf

`rustecal-types-protobuf` provides a helper wrapper for Protobuf messages (using `prost`) to use with the typed eCAL Pub/Sub API.

## Features

- **ProtobufMessage<T>**: wrap and transport Protobuf messages  
- Implements `PublisherMessage` and `SubscriberMessage` for seamless integration  
- Zero-copy where possible via `Arc::from(ProtobufMessage)`
- Static descriptor embedding via `include_bytes!` (optional)  
- No extra dependencies beyond `prost`, `rustecal-core` and `rustecal-pubsub`  

## Installation

Add to your **workspace** `Cargo.toml`:

```toml
[dependencies]
rustecal-types-protobuf = "0.1"
```

## Usage

### Publisher Example

```rust
use std::sync::Arc;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};

mod people      { include!(concat!(env!("OUT_DIR"), "/pb.people.rs")); }
mod animal      { include!(concat!(env!("OUT_DIR"), "/pb.animal.rs")); }
mod environment { include!(concat!(env!("OUT_DIR"), "/pb.environment.rs")); }

use people::Person;
impl IsProtobufType for Person {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("protobuf publisher"), EcalComponents::DEFAULT)?;

    let publisher = TypedPublisher::<ProtobufMessage<Person>>::new("person")?;

    while Ecal::ok() {
        let person = Person { id: 1, name: "Alice".into(), ..Default::default() };

        let message = ProtobufMessage { data : Arc::from(person) };
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
use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};

mod people      { include!(concat!(env!("OUT_DIR"), "/pb.people.rs")); }
mod animal      { include!(concat!(env!("OUT_DIR"), "/pb.animal.rs")); }
mod environment { include!(concat!(env!("OUT_DIR"), "/pb.environment.rs")); }

use people::Person;
impl IsProtobufType for Person {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("protobuf subscriber"), EcalComponents::DEFAULT)?;

    let mut subscriber = TypedSubscriber::<ProtobufMessage<Person>>::new("person")?;
    subscriber.set_callback(|message| {
        println!("Received person: {}", message.payload.data.name)
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
- `rustecal-types-string` for UTF-8 string messages
- `rustecal-types-serde` for JSON/CBOR/MessagePack via Serde
- Examples in the `rustecal-samples/pubsub` directory
