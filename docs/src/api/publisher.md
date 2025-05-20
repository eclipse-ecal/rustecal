# Typed Publisher

The `Publisher<T>` allows you to publish messages of type `T` on a topic.

## Example

```rust
use std::sync::Arc;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

let publisher = TypedPublisher::<StringMessage>::new("hello").unwrap();

let message = StringMessage { data: Arc::from("Hello from Rust") };
publisher.send(&message);
```
