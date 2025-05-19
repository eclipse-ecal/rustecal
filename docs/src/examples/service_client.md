# Service Client Example

This example demonstrates how to call a **Mirror Service** using `rustecal`.

## Example Code

```rust
use rustecal::{Ecal, EcalComponents, ServiceClient, ServiceRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("mirror_client"), EcalComponents::DEFAULT)?;

    let client = ServiceClient::new("mirror_service")?;

    let request_data = b"Hello, Service!";
    let timeout = Some(500);

    while Ecal::ok() {
        let request = ServiceRequest {
            payload: request_data.to_vec(),
        };

        // Call the "mirror" method
        if let Some(response) = client.call("mirror", request, timeout) {
            // Extract the echoed payload
            let echoed = String::from_utf8_lossy(&response.payload);
            println!("Received response: {}", echoed);
        } else {
            println!("Service call timed out.");
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```
