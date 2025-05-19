# Typed Publisher

The `Publisher<T>` allows you to publish messages of type `T` on a topic.

## Example

```rust
use rustecal::pubsub::Publisher;
use rustecal_types::StringMessage;

let publisher = Publisher::<StringMessage>::builder("my_topic").create()?;
publisher.send("Rust rocks!")?;
```
