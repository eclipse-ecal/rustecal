use crate::format_support::{short_type_name, FormatSupport};
use crate::make_format;
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// JSON support using `serde_json`.
#[derive(Debug, Clone)]
pub struct JsonSupport;
impl FormatSupport for JsonSupport {
    const ENCODING: &'static str = "json";
    fn encode<T: Serialize>(payload: &T) -> Vec<u8> {
        serde_json::to_vec(payload).expect("JSON serialization failed")
    }
    fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Option<T> {
        serde_json::from_slice(bytes).ok()
    }
}

make_format!(JsonMessage, JsonSupport);

impl<T> PublisherMessage for JsonMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: JsonSupport::ENCODING.into(),
            type_name: short_type_name::<T>(),
            descriptor: vec![],
        }
    }
    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(JsonSupport::encode(&*self.data))
    }
}
impl<T> SubscriberMessage<'_> for JsonMessage<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn datatype() -> DataTypeInfo {
        <JsonMessage<T> as PublisherMessage>::datatype()
    }
    fn from_bytes(bytes: &[u8], _dt: &DataTypeInfo) -> Option<Self> {
        JsonSupport::decode(bytes).map(|p| JsonMessage { data: Arc::new(p) })
    }
}
