use crate::format_support::{short_type_name, FormatSupport};
use crate::make_format;
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// CBOR support using `serde_cbor`.
#[derive(Debug, Clone)]
pub struct CborSupport;
impl FormatSupport for CborSupport {
    const ENCODING: &'static str = "cbor";
    fn encode<T: Serialize>(payload: &T) -> Vec<u8> {
        serde_cbor::to_vec(payload).expect("CBOR serialization failed")
    }
    fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Option<T> {
        serde_cbor::from_slice(bytes).ok()
    }
}

make_format!(CborMessage, CborSupport);

impl<T> PublisherMessage for CborMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: CborSupport::ENCODING.into(),
            type_name: short_type_name::<T>(),
            descriptor: vec![],
        }
    }
    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(CborSupport::encode(&*self.data))
    }
}
impl<T> SubscriberMessage<'_> for CborMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <CborMessage<T> as PublisherMessage>::datatype()
    }
    fn from_bytes(bytes: &[u8], _dt: &DataTypeInfo) -> Option<Self> {
        CborSupport::decode(bytes.as_ref()).map(|p| CborMessage { data: Arc::new(p) })
    }
}
