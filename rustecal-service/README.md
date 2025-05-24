# rustecal-service

`rustecal-service` provides a high-level RPC-style service server and client API, enabling request-response interactions with minimal boilerplate.

## Features

- **ServiceServer**: host one or more methods, register handlers via closures
- **ServiceClient**: invoke remote methods with optional timeouts
- **Method metadata** (`MethodInfo`) and structured responses (`ServiceResponse`)
- Built-in error handling and call-state reporting

## Requirements

- Rust 1.60 or later  
- eCAL C/C++ library v6.0 or later installed on your system  

## Installation

Add to your workspace `Cargo.toml`:

```toml
[dependencies]
rustecal-service = "0.1"
```

## Quickstart

### Server Example

```rust
use rustecal::{Ecal, EcalComponents, ServiceServer};
use rustecal::service::types::MethodInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ecal::initialize(Some("mirror_server"), EcalComponents::DEFAULT)?;

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


### Client Example

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

## Traits Reference

- **`ServiceServer`**
  - `new(topic: &str) -> Result<Self, String>`
  - `add_method(method: &str, callback: ServiceCallback) -> Result<(), String>`

- **`ServiceClient`**
  - `new(service_name: &str) -> Result<Self, String>`
  - `call(method: &str, req: ServiceRequest, timeout_ms: Option<i32>) -> Option<ServiceResponse>`

## See Also

- Examples in the `rustecal-samples/service` directory  
