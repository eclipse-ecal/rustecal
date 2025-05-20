# Typed Subscriber

The `Subscriber<T>` enables you to subscribe to messages of type `T` on a topic.

## Example

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;

let mut subscriber = TypedSubscriber::<StringMessage>::new("hello")?;
subscriber.set_callback(|message| {
    println!("Received: {}", message.payload.data)
```
