use rustecal::{Ecal, EcalComponents, TypedPublisher};
use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};

mod people { include!(concat!(env!("OUT_DIR"), "/pb.people.rs")); }
mod animal { include!(concat!(env!("OUT_DIR"), "/pb.animal.rs")); }
mod environment { include!(concat!(env!("OUT_DIR"), "/pb.environment.rs")); }

use people::Person;
impl IsProtobufType for Person {}

fn main() {
    Ecal::initialize(Some("person send rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let publisher = TypedPublisher::<ProtobufMessage<Person>>::new("person")
        .expect("Failed to create publisher");

    let mut cnt = 0;
    while Ecal::ok() {
        cnt += 1;

        let person = Person {
            id: cnt,
            name: "Max".to_string(),
            stype: 1,
            email: "max@mail.net".to_string(),
            dog: Some(animal::Dog {
                name: "Brandy".to_string(),
                colour: "Brown".to_string(),
            }),
            house: Some(environment::House {
                rooms: 4,
            }),
        };

        // Wrap the person struct in ProtobufMessage
        let wrapped = ProtobufMessage(person.clone());
        publisher.send(&wrapped);

        println!("person id    : {}", person.id);
        println!("person name  : {}", person.name);
        println!("person stype : {}", person.stype);
        println!("person email : {}", person.email);
        println!("dog.name     : {}", person.dog.as_ref().map_or("", |d| &d.name));
        println!("house.rooms  : {}", person.house.as_ref().map_or(0, |h| h.rooms));
        println!();

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
