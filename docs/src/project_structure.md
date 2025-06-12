# Project Structure

This workspace is organized into several purpose-specific crates to provide a modular, maintainable API for eCAL:

| Crate                     | Description                                                                                                      |
|---------------------------|------------------------------------------------------------------------------------------------------------------|
| `rustecal`                | **Meta-crate**: re-exports core, pub/sub, and service APIs via feature flags (`pubsub`, `service`)               |
| `rustecal-core`           | Core lifecycle management, logging, monitoring, error handling, and shared type definitions                      |
| `rustecal-pubsub`         | Typed and untyped Publisher/Subscriber API                                                                       |
| `rustecal-service`        | RPC service server & client API                                                                                  |
| `rustecal-sys`            | Low-level FFI bindings to the eCAL C API                                                                         |
| `rustecal-types-string`   | Helper: UTF-8 string message wrapper for typed pub/sub                                                           |
| `rustecal-types-bytes`    | Helper: raw byte vector message wrapper                                                                          |
| `rustecal-types-protobuf` | Helper: Protobuf message wrapper (using `prost`)                                                                 |
| `rustecal-types-serde`    | Helper: Serde JSON/CBOR/MessagePack message wrappers for typed pub/sub                                           |
| `rustecal-samples`        | Example binaries demonstrating pub/sub, RPC, monitoring, and logging                                             |
