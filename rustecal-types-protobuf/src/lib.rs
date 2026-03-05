//! # rustecal-types-protobuf
//!
//! Provides support for Protobuf message serialization with rustecal.

use prost::Message;
use prost_reflect::{FileDescriptor, ReflectMessage};
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use std::collections::HashSet;
use std::sync::Arc;

/// Marker trait to opt-in a Protobuf type for use with eCAL.
///
/// This trait must be implemented for any `prost::Message` you wish to use
/// with `ProtobufMessage<T>`. It provides a type-level opt-in mechanism
/// to ensure users are aware of what's being exposed to eCAL.
pub trait IsProtobufType {}

/// A wrapper for protobuf messages used with typed eCAL pub/sub.
///
/// This type allows sending and receiving protobuf messages through the
/// `TypedPublisher` and `TypedSubscriber` APIs.
#[derive(Debug, Clone)]
pub struct ProtobufMessage<T> {
    pub data: Arc<T>,
}

impl<T> SubscriberMessage<'_> for ProtobufMessage<T>
where
    T: Message + Default + IsProtobufType + ReflectMessage,
{
    /// Returns metadata used by eCAL to describe the Protobuf type.
    ///
    /// This includes:
    /// - `proto` as encoding
    /// - the Rust type name
    /// - an optional descriptor
    fn datatype() -> DataTypeInfo {
        let default_instance = T::default();
        let instance_descriptor = default_instance.descriptor();
        let instance_file = instance_descriptor.parent_file();
        let type_name = instance_descriptor.full_name().to_string();

        // Loop through all the dependencies of T
        fn process_protos_recursively(file: &FileDescriptor, visited: &mut HashSet<String>) {
            let name = file.name().to_string();

            if visited.contains(&name) {
                return;
            }

            visited.insert(name);

            for dep in file.dependencies() {
                process_protos_recursively(&dep, visited);
            }
        }

        // Filenames of protos for T
        let mut visited = HashSet::new();
        process_protos_recursively(&instance_file, &mut visited);

        // Collect a filtered list of FileDescriptors for T
        let file_descriptors: Vec<FileDescriptor> = instance_descriptor
            .parent_pool()
            .files()
            .filter(|s| visited.contains(s.name()))
            .collect();

        let mut descriptor_pool = prost_reflect::DescriptorPool::new();

        // Add to the file descriptor pool
        for proto_file in file_descriptors {
            let mut file_descriptor_proto = proto_file.file_descriptor_proto().clone();
            // Remove the source_code_info from the descriptor which add excess comments
            // from original proto to the descriptor message that aren't needed
            file_descriptor_proto.source_code_info = None;

            descriptor_pool
                .add_file_descriptor_proto(file_descriptor_proto)
                .expect("Unable to add protobuf to pool");
        }

        DataTypeInfo {
            encoding: "proto".to_string(),
            type_name,
            descriptor: descriptor_pool.encode_to_vec(),
        }
    }

    /// Decodes a Protobuf message from bytes.
    ///
    /// # Returns
    /// - `Some(ProtobufMessage<T>)` on success
    /// - `None` if decoding fails
    fn from_bytes(bytes: &[u8], _data_type_info: &DataTypeInfo) -> Option<Self> {
        T::decode(bytes).ok().map(|msg| ProtobufMessage {
            data: Arc::new(msg),
        })
    }
}

impl<T> PublisherMessage for ProtobufMessage<T>
where
    T: Message + Default + IsProtobufType + ReflectMessage,
{
    /// Returns the same datatype information as [`SubscriberMessage`]
    /// implementation.
    fn datatype() -> DataTypeInfo {
        <ProtobufMessage<T> as SubscriberMessage>::datatype()
    }

    /// Encodes the message to a byte buffer.
    ///
    /// # Panics
    /// Will panic if `prost::Message::encode` fails (should never panic for
    /// valid messages).
    fn to_bytes(&self) -> Arc<[u8]> {
        let mut buf = Vec::with_capacity(self.data.encoded_len());
        self.data
            .encode(&mut buf)
            .expect("Failed to encode protobuf message");
        Arc::from(buf)
    }
}

#[cfg(test)]
pub static DESCRIPTOR_POOL: once_cell::sync::Lazy<prost_reflect::DescriptorPool> =
    once_cell::sync::Lazy::new(|| {
        prost_reflect::DescriptorPool::decode(
            include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
        )
        .unwrap()
    });

#[cfg(test)]
pub mod tests {
    use super::*;

    use prost::Name;

    // Include protobuf mod definitions
    include!(concat!(env!("OUT_DIR"), "/_include.rs"));

    use example::msg::basic::Basic;
    use example::msg::nested::Nested;

    impl IsProtobufType for Basic {}
    impl IsProtobufType for Nested {}

    #[test]
    fn basic_proto() {
        type TestProto = Basic;

        let datainfo =
            <ProtobufMessage<TestProto> as rustecal_pubsub::PublisherMessage>::datatype();

        assert!(datainfo.type_name == TestProto::full_name());
        assert!(datainfo.encoding == "proto");
    }

    #[test]
    // This would fail on <= 0.1.6
    // called `Result::unwrap()` on an `Err` value: nested/status.proto: imported file 'google/protobuf/any.proto' has not been added
    fn nested_proto() {
        type TestProto = Nested;

        let datainfo =
            <ProtobufMessage<TestProto> as rustecal_pubsub::PublisherMessage>::datatype();

        assert!(datainfo.type_name == TestProto::full_name());
        assert!(datainfo.encoding == "proto");
    }
}
