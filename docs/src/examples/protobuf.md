# Protobuf Message Example

## Publisher

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
        publisher.send(&ProtobufMessage(Arc::from(person)));

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```

## Subscriber

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
    subscriber.set_callback(|msg| println!("Received person: {}", msg.msg.0.name));

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```
