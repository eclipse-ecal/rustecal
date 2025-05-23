use std::sync::Arc;
use serde::{Deserialize, Serialize};
use rustecal_core::types::DataTypeInfo;
use rustecal_pubsub::typed_publisher::PublisherMessage;
use rustecal_pubsub::typed_subscriber::SubscriberMessage;
use crate::format_support::{FormatSupport, short_type_name};
use crate::make_format;

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
where T: Serialize + for<'de> Deserialize<'de> + Clone
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo { encoding: JsonSupport::ENCODING.into(), type_name: short_type_name::<T>(), descriptor: vec![] }
    }
    fn to_bytes(&self) -> Arc<[u8]> {
        Arc::from(JsonSupport::encode(&*self.data))
    }
}
impl<T> SubscriberMessage for JsonMessage<T>
where T: Serialize + for<'de> Deserialize<'de> + Clone
{
    fn datatype() -> DataTypeInfo { <JsonMessage<T> as PublisherMessage>::datatype() }
    fn from_bytes(bytes: Arc<[u8]>, _dt: &DataTypeInfo) -> Option<Self> {
        JsonSupport::decode(bytes.as_ref()).map(|p| JsonMessage { data: Arc::new(p) })
    }
}
