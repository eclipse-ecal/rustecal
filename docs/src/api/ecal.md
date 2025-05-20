# Ecal Lifecycle

The `Ecal` struct manages initialization and finalization of the eCAL system.

## Example

```rust
use rustecal::{Ecal, EcalComponents};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("my ecal app"), EcalComponents::DEFAULT)?;

    // use publishers, subscribers, clients, server

    Ecal::finalize();
    Ok(())
}
```
