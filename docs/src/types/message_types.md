# Supported Message Types

- `BytesMessage` – Arbitrary binary data (`rustecal-types-bytes`)
- `StringMessage` – UTF-8 encoded strings (`rustecal-types-string`)
- `ProtobufMessage<T>` – Protobuf messages (`rustecal-types-protobuf`)
- `JsonMessage<T>` – JSON-serialized Serde types (`rustecal-types-serde`)
- `CborMessage<T>` – CBOR-serialized Serde types (`rustecal-types-serde`)
- `MsgpackMessage<T>` – MessagePack-serialized Serde types (`rustecal-types-serde`)

Each type is provided via a dedicated crate to avoid pulling unnecessary dependencies.
