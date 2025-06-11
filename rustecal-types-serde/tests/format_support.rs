use rustecal_types_serde::format_support;
use rustecal_types_serde::json_message::JsonSupport;

#[test]
fn short_type_name_for_implementor() {
    assert_eq!(format_support::short_type_name::<JsonSupport>(), "JsonSupport");
}

#[test]
fn short_type_name_for_nested_type() {
    mod nested {
        pub mod deep {
            pub struct TestType;
        }
    }
    assert_eq!(format_support::short_type_name::<nested::deep::TestType>(), "TestType");
}
