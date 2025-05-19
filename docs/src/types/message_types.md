# Supported Message Types

- `BytesMessage` – Arbitrary binary data
- `StringMessage` – UTF-8 encoded strings
- `ProtobufMessage<T>` – Protobuf messages

Each type is provided via a dedicated crate to avoid pulling unnecessary dependencies.
