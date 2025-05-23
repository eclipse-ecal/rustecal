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

## Workspace Layout

```text
your_workspace/
├── Cargo.toml                # workspace manifest
├── rustecal/                 # meta-crate
├── rustecal-core/            # core init, logging, monitoring, error, shared types
│   └── src/
│       ├── core.rs
│       ├── error.rs
│       ├── log.rs
│       ├── log_level.rs
│       ├── monitoring.rs
│       └── core_types/
│           ├── logging.rs
│           └── monitoring.rs
├── rustecal-pubsub/          # pub/sub API
├── rustecal-service/         # service RPC API
├── rustecal-sys/             # raw C bindings
├── rustecal-types-string/    # UTF-8 string pub/sub
├── rustecal-types-bytes/     # raw bytes pub/sub
├── rustecal-types-protobuf/  # Protobuf pub/sub
├── rustecal-types-serde/     # Serde JSON/CBOR/MsgPack pub/sub
└── rustecal-samples/         # examples
    ├── pubsub/
    │   ├── hello_send/
    │   ├── hello_receive/
    │   ├── blob_send/
    │   ├── blob_receive/
    │   ├── person_send/
    │   └── person_receive/
    ├── service/
    │   ├── mirror_server/
    │   ├── mirror_client/
    │   └── mirror_client_instances/
    ├── monitoring_receive/
    └── logging_receive/
