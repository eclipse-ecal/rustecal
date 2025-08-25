fn main() {
    let protos = [
        "proto/person.proto",
        "proto/animal.proto",
        "proto/house.proto",
    ];

    let protos_inc = ["proto"];

    prost_build::compile_protos(&protos, &protos_inc).unwrap();

    prost_reflect_build::Builder::new()
        .descriptor_pool("crate::DESCRIPTOR_POOL")
        .compile_protos(&protos, &protos_inc)
        .unwrap();
}
