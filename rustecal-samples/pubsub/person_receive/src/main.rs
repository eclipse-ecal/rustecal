use rustecal::{Ecal, EcalComponents, TypedSubscriber};
use rustecal::pubsub::typed_subscriber::Received;
use rustecal_types_protobuf::{ProtobufMessage, IsProtobufType};

mod people { include!(concat!(env!("OUT_DIR"), "/pb.people.rs")); }
mod animal { include!(concat!(env!("OUT_DIR"), "/pb.animal.rs")); }
mod environment { include!(concat!(env!("OUT_DIR"), "/pb.environment.rs")); }

use people::Person;
impl IsProtobufType for Person {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize eCAL
    Ecal::initialize(Some("person receive rust"), EcalComponents::DEFAULT)
        .expect("eCAL initialization failed");

    let mut subscriber = TypedSubscriber::<ProtobufMessage<Person>>::new("person")?;

    subscriber.set_callback(|msg: Received<ProtobufMessage<Person>>| {
        let person = msg.payload.data;

        println!("------------------------------------------");
        println!(" MESSAGE HEAD ");
        println!("------------------------------------------");
        println!("topic name   : {}", msg.topic_name);
        println!("encoding     : {}", msg.encoding);
        println!("type name    : {}", msg.type_name);
        println!("topic time   : {}", msg.timestamp);
        println!("topic clock  : {}", msg.clock);
        println!("------------------------------------------");
        println!(" MESSAGE CONTENT ");
        println!("------------------------------------------");
        println!("person id    : {}", person.id);
        println!("person name  : {}", person.name);
        println!("person stype : {}", person.stype);
        println!("person email : {}", person.email);
        println!("dog.name     : {}", person.dog.as_ref().map_or("", |d| &d.name));
        println!("house.rooms  : {}", person.house.as_ref().map_or(0, |h| h.rooms));
        println!("------------------------------------------\n");
    });

    println!("Waiting for messages on topic 'person'...");

    // keep the thread alive so callbacks can run
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // clean up and finalize eCAL
    Ecal::finalize();
    Ok(())
}
