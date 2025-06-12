# Service Server Example

This example shows how to implement a basic **Mirror Service Server** using `rustecal`.

The server receives a request, logs it, and sends it back as a response.

## Example Code

```rust
use rustecal::{Ecal, EcalComponents, ServiceServer};
use rustecal::service::types::MethodInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("mirror_server"), EcalComponents::DEFAULT, None)?;

    let mut server = ServiceServer::new("mirror_service")?;

    server.add_method("mirror", Box::new(|method: MethodInfo, req: &[u8]| {
        let request_str = String::from_utf8_lossy(req);
        println!("Received [{}] request: {}", method.method_name, request_str);

        // Echo (mirror) the same bytes back
        req.to_vec()
    }))?;

    println!("mirror_service is running…");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
    Ok(())
}
```
