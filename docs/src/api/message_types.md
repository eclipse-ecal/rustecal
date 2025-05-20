# Supported Message Types

`rustecal` supports message types through wrapper structs:

## `StringMessage`

Used for UTF-8 string topics.

## `BytesMessage`

Used for binary `Vec<u8>` payloads.

## `ProtobufMessage<T>`

Supports publishing/receiving of Protobuf types that implement `Message` and `Default`.

```rust
use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};

use people::Person;
impl IsProtobufType for Person {}

let publisher = TypedPublisher::<ProtobufMessage<Person>>::new("person").unwrap();
```
