# Typed Publisher

The `Publisher<T>` allows you to publish messages of type `T` on a topic.

## Example

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal::pubsub::publisher::Timestamp;
use rustecal_types_string::StringMessage;

let publisher = TypedPublisher::<StringMessage>::new("hello").unwrap();

let message = StringMessage { data: "Hello from Rust".into() }
publisher.send(&message, Timestamp::Auto);
```
