/// Defines a serialization format adapter for Serde payloads.
pub trait FormatSupport {
    /// The encoding label for DataTypeInfo.
    const ENCODING: &'static str;
    /// Serialize the payload to bytes.
    fn encode<T: serde::Serialize>(payload: &T) -> Vec<u8>;
    /// Deserialize the payload from bytes.
    fn decode<T: for<'de> serde::Deserialize<'de>>(bytes: &[u8]) -> Option<T>;
}

/// Helper to extract the short Rust type name without module prefixes.
pub fn short_type_name<T>() -> String {
    let full = std::any::type_name::<T>();
    full.rsplit("::").next().unwrap_or(full).to_string()
}

/// Macro to generate format-specific message wrappers with a public `data` field.
#[macro_export]
macro_rules! make_format {
    ($msg_type:ident, $support:ty) => {
        #[derive(Debug, Clone)]
        pub struct $msg_type<T>
        where T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone
        {
            /// The inner payload.
            pub data: std::sync::Arc<T>,
        }
        impl<T> $msg_type<T>
        where T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone
        {
            /// Create a new message with given payload.
            pub fn new(payload: T) -> Self {
                $msg_type { data: std::sync::Arc::new(payload) }
            }
        }
    };
}
