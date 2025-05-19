# Binary Message Example

## Publisher

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
        publisher.send(&BytesMessage(Arc::from(buf)));

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
    Ecal::initialize(Some("blob subscriber"), EcalComponents::DEFAULT)?;

    let mut subscriber = TypedSubscriber::<BytesMessage>::new("blob")?;
    subscriber.set_callback(|msg| {
        println!("Received blob of {} bytes", msg.payload.data.len());
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```
