use std::sync::Arc;
use serde::{Deserialize, Serialize};
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use crate::format_support::{FormatSupport, short_type_name};
use crate::make_format;

/// MessagePack support using `rmp-serde`.
#[derive(Debug, Clone)]
pub struct MsgpackSupport;
impl FormatSupport for MsgpackSupport {
    const ENCODING: &'static str = "msgpack";
    fn encode<T: Serialize>(payload: &T) -> Vec<u8> {
        rmp_serde::to_vec(payload).expect("MessagePack serialization failed")
    }
    fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Option<T> {
        rmp_serde::from_slice(bytes).ok()
    }
}

make_format!(MsgpackMessage, MsgpackSupport);

impl<T> PublisherMessage for MsgpackMessage<T>
where T: Serialize + for<'de> Deserialize<'de> + Clone
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo { encoding: MsgpackSupport::ENCODING.into(), type_name: short_type_name::<T>(), descriptor: vec![] }
    }
    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(MsgpackSupport::encode(&*self.data))
    }
}
impl<T> SubscriberMessage for MsgpackMessage<T>
where T: Serialize + for<'de> Deserialize<'de> + Clone
{
    fn datatype() -> DataTypeInfo { <MsgpackMessage<T> as PublisherMessage>::datatype() }
    fn from_bytes(bytes: Arc<[u8]>, _dt: &DataTypeInfo) -> Option<Self> {
        MsgpackSupport::decode(bytes.as_ref()).map(|p| MsgpackMessage { data: Arc::new(p) })
    }
}
