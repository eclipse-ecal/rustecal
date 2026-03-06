fn main() {
    let protos = ["./protos/basic.proto", "./protos/nested.proto"];

    let protos_inc = ["./protos"];

    let mut config = prost_build::Config::new();
    config.include_file("_include.rs"); // Generate mod import file
    config.enable_type_names();

    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .compile_protos_with_config(config, &protos, &protos_inc)
        .unwrap();
}
