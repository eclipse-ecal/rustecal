fn main() {
    prost_build::compile_protos(
        &[
            "proto/person.proto",
            "proto/animal.proto",
            "proto/house.proto",
        ],
        &["proto"],
    )
    .unwrap();
}
