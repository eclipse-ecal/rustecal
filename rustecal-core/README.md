# rustecal-core

**Idiomatic Rust API for Eclipse eCAL**

`rustecal-core` provides high-level Rust abstractions for eCAL’s pub/sub and service/client patterns, built on top of the low-level FFI bindings in [`rustecal-sys`](https://crates.io/crates/rustecal-sys).

---

## Features

- **Core Initialization & Lifecycle**: Initialize, finalize, retrieve version information, component selection via `EcalComponents` bitflags.
- **Configuration**: Flexible configuration via environment variables and builder patterns.
- **Monitoring**: Inspect the eCAL runtime state including process, topic, and service/client details.
- **Logging**: Emit and retrieve log messages at various severity levels.
- **Error Handling**: Comprehensive `RustecalError` enum for FFI errors and internal issues.

## Requirements

- **Rust** 1.60 or later  
- **eCAL** C/C++ library v6.0 or later installed and accessible on your system

## Installation

Add `rustecal-core` to your `Cargo.toml`:

```toml
[dependencies]
rustecal-core = "0.1"
```
