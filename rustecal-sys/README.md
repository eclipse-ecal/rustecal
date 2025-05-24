# rustecal-sys

Low-level FFI bindings for the [Eclipse eCAL](https://github.com/eclipse-ecal/ecal) C++ library, enabling Rust code to interface directly with eCAL’s core functionality.

## Features

- **Rust FFI**: Safe Rust wrappers live in higher-level crates (`rustecal-core`), while `rustecal-sys` provides the raw `extern "C"` declarations.
- **Cross-platform**: Supports Linux, Windows, and macOS (provided the eCAL C/C++ library is installed for your platform).
- **Build script**: Auto-detects eCAL installations.

## Requirements

- **Rust** 1.60 or later
- **Eclipse eCAL** C/C++ library v6.0 or later installed and accessible on your system
