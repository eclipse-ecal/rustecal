use prost_reflect::DescriptorPool;
use std::sync::LazyLock;

pub static DESCRIPTOR_POOL: LazyLock<DescriptorPool> = LazyLock::new(|| {
    DescriptorPool::decode(
        include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
    )
    .unwrap()
});

// Added generated mod paths
include!(concat!(env!("OUT_DIR"), "/_include.rs"));
