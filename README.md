# rustecal

Rust bindings for the high-performance [eCAL](https://github.com/eclipse-ecal/ecal) middleware, providing efficient pub/sub and service-based communication for interprocess and distributed systems.

---

## Features

- Idiomatic Rust interface to the eCAL API
- Zero-copy shared memory transport
- Type-safe publish/subscribe and service communication
- Modular type support: String, Binary, Protobuf
- Fully runtime-compatible with C++ eCAL systems

---

## Examples

### Publisher

```rust
use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_string::StringMessage;

fn main() {
    Ecal::initialize(Some("hello publisher"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher: TypedPublisher<StringMessage> = TypedPublisher::<StringMessage>::new("chatter")
        .expect("Failed to create publisher");

    while Ecal::ok() {
        let wrapped = StringMessage("Hello from Rust!".to_string());
        publisher.send(&wrapped);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
```

---

### Subscriber

```rust
use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal_types_string::StringMessage;
use rustecal::pubsub::typed_subscriber::Received;

fn main() {
    Ecal::initialize(Some("hello subscriber"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<StringMessage>::new("chatter")
        .expect("Failed to create subscriber");

    subscriber.set_callback(|msg: Received<StringMessage>| {
        println!("Received : {}", msg.msg.0);
    });

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ecal::finalize();
}
```

---

### Service Server

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal::service::server::ServiceServer;
use rustecal::service::types::MethodInfo;

fn main() {
    Ecal::initialize(Some("mirror server"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut server = ServiceServer::new("mirror")
        .expect("Failed to create server");

    server.add_method("reverse", Box::new(|_info: MethodInfo, req: &[u8]| {
        let mut reversed = req.to_vec();
        reversed.reverse();
        reversed
    })).unwrap();

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ecal::finalize();
}
```

---

### Service Client

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal::service::client::ServiceClient;
use rustecal::service::types::ServiceRequest;

fn main() {
    Ecal::initialize(Some("mirror client"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let client = ServiceClient::new("mirror")
        .expect("Failed to create client");

    while Ecal::ok() {
        let request = ServiceRequest {
            payload: b"stressed".to_vec(),
        };

        if let Some(response) = client.call("reverse", request, Some(1000)) {
            println!("Reversed: {}", String::from_utf8_lossy(&response.payload));
        } else {
            println!("No response received.");
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

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

Licensed under Apache-2.0 or MIT.

---

## Maintainer

[Rex Schilasky](https://github.com/rex-schilasky)
