# rustecal

Rust bindings for the high-performance [eCAL](https://github.com/eclipse-ecal/ecal) middleware, providing efficient pub/sub and service-based communication for interprocess and distributed systems.

---

## Features

- Idiomatic Rust interface to the eCAL API
- Zero-copy shared memory transport
- Type-safe publish/subscribe and service communication
- Modular type support: String, Binary, Protobuf, JSON, CBOR, MessagePack
- Fully runtime-compatible with C++ eCAL systems

---

## System Requirements

This crate requires a native installation of the [Eclipse eCAL](https://github.com/eclipse-ecal/ecal) C/C++ runtime, version 6.0 or newer. General instructions for installation can be found [here](https://eclipse-ecal.github.io/ecal/stable/getting_started/setup.html). The crate will fail to compile if the native libraries are not found.

---

## Examples

### Publisher

```rust
use std::time::Duration;
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal::pubsub::publisher::Timestamp;
use rustecal_types_string::StringMessage;

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("hello publisher"), EcalComponents::DEFAULT, None).unwrap();

    // create a string publisher on "hello"
    let publisher = TypedPublisher::<StringMessage>::new("hello").unwrap();

    // prepare the message to send
    let message = StringMessage { data: "Hello from Rust".into() };

    // publish until eCAL shuts down
    while Ecal::ok() {
        publisher.send(&message, Timestamp::Auto);
        std::thread::sleep(Duration::from_millis(500));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
}
```

---

### Subscriber

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("hello subscriber"), EcalComponents::DEFAULT, None).unwrap();

    // create a string subscriber on “hello”
    let mut subscriber = TypedSubscriber::<StringMessage>::new("hello").unwrap();

    // print each incoming message
    subscriber.set_callback(|message| {
        println!("Received: {}", message.payload.data)
    });

    // keep the thread alive so callbacks can run
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
}
```

---

### Service Server

```rust
use std::time::Duration;
use rustecal::{Ecal, EcalComponents};
use rustecal::service::server::ServiceServer;
use rustecal::service::types::MethodInfo;

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("mirror server"), EcalComponents::DEFAULT, None).unwrap();

    // create a service server for "mirror"
    let mut server = ServiceServer::new("mirror").unwrap();

    // register the "reverse" method
    server
        .add_method(
            "reverse",
            Box::new(|_info: MethodInfo, req: &[u8]| {
                let mut reversed = req.to_vec();
                reversed.reverse();
                reversed
            }),
        )
        .unwrap();

    // keep the server alive to handle incoming calls
    while Ecal::ok() {
        std::thread::sleep(Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
}
```

---

### Service Client

```rust
use std::time::Duration;
use rustecal::{Ecal, EcalComponents};
use rustecal::service::client::ServiceClient;
use rustecal::service::types::ServiceRequest;

fn main() {
    // Initialize eCAL
    Ecal::initialize(Some("mirror client"), EcalComponents::DEFAULT, None).unwrap();

    // create a service client for "mirror"
    let client = ServiceClient::new("mirror").unwrap();

    // call the "reverse" service until eCAL shuts down
    while Ecal::ok() {
        // prepare the request payload
        let request = ServiceRequest {
            payload: b"stressed".to_vec(),
        };

        // send the request and print the response if any
        if let Some(response) = client.call("reverse", request, Some(1000)) {
            println!("Reversed: {}", String::from_utf8_lossy(&response.payload));
        } else {
            println!("No response received.");
        }

        // throttle the request rate
        std::thread::sleep(Duration::from_secs(1));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
}
```

---

## Documentation

Full user guide: [https://eclipse-ecal.github.io/rustecal](https://eclipse-ecal.github.io/rustecal)

```bash
cd docs/
mdbook serve
```

---

## License

Licensed under Apache-2.0.

---

## Maintainer

[Rex Schilasky](https://github.com/rex-schilasky)
