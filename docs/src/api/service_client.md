# Service Client

The `ServiceClient` API allows a Rust application to call eCAL services, either generically or per-instance.

## Connecting to a Service

```rust
use rustecal::service::client::ServiceClient;

let client = ServiceClient::new("mirror")?;
```

## Calling Methods

```rust
use rustecal::service::types::ServiceRequest;

let request = ServiceRequest {
    payload: b"stressed".to_vec(),
};

let response = client.call("echo", request, Some(1000));
```

To call all connected instances:

```rust
for instance in client.get_client_instances() {
    let response = instance.call("reverse", request.clone(), Some(1000));
}
```

## Return Handling

```rust
match response {
    Some(res) if res.success => {
        println!("Response: {}", String::from_utf8_lossy(&res.payload));
    }
    Some(res) => {
        println!("Error: {}", res.error_msg.unwrap_or("Unknown error".into()));
    }
    None => {
        println!("No response or timeout.");
    }
}
```

## Runtime Compatibility

This API is fully compatible with the C++ `mirror_client.cpp` and C `mirror_client_c.c` examples.