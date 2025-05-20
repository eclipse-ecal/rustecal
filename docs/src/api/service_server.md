# Service Server

The `ServiceServer` API allows Rust applications to act as eCAL service providers using a simple, callback-based interface that mirrors the C++ and C APIs.

## Registering Methods

To provide services, create a `ServiceServer` and register one or more methods by name:

```rust
use rustecal::service::server::ServiceServer;
use rustecal::service::types::MethodInfo;

let mut server = ServiceServer::new("mirror")?;

server.add_method("echo", Box::new(|_info: MethodInfo, request: &[u8]| {
    request.to_vec()
}))?;

server.add_method("reverse", Box::new(|_info, request| {
    let mut reversed = request.to_vec();
    reversed.reverse();
    reversed
}))?;
```

## Method Signatures

The callback signature follows:

```rust
Fn(MethodInfo, &[u8]) -> Vec<u8>
```

This is safe, allocation-free on the input side, and flexible for any binary or textual payloads.

## Example Output

```
Method   : 'echo' called
Request  : stressed
Response : stressed

Method   : 'reverse' called
Request  : stressed
Response : desserts
```

## Runtime Compatibility

This API is fully compatible with the C++ `mirror_server.cpp`, the C `mirror_server_c.c` and the C# `mirror_client_csharp.cs` example.
