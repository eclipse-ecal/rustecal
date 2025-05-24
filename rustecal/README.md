# rustecal

`rustecal` is the meta-crate for the Rust eCAL ecosystem. It re-exports:

- Core initialization & lifecycle (`rustecal-core`)  
- (Typed) Publish/Subscribe API (`rustecal-pubsub`, optional)  
- RPC-style Server/Client (`rustecal-service`, optional)  

via Cargo feature flags.

## Features

| Feature   | Crate                          | Description                          |
|-----------|--------------------------------|--------------------------------------|
| `default` | `core`, `pubsub`, `service`    | All functionality enabled            |
| `pubsub`  | `rustecal-pubsub` (optional)   | (Typed) Publish/Subscribe API        |
| `service` | `rustecal-service` (optional)  | RPC-style Server/Client API          |

## Requirements

- **Rust** 1.60 or later  
- **Eclipse eCAL** C/C++ library v6.0 or later installed and accessible on your system

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rustecal = "0.1"
```

By default, rustecal enables both pubsub and service. To disable one or both:

```toml
[dependencies]
rustecal = { version = "0.1", default-features = false, features = ["pubsub"] }
```

## See Also

- [`rustecal-core`](https://docs.rs/rustecal-core) – core init, logging, monitoring
- [`rustecal-pubsub`](https://docs.rs/rustecal-pubsub) – Publish/Subscribe API
- [`rustecal-service`](https://docs.rs/rustecal-service) – RPC-style Service API
