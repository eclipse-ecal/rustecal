fn main() {
    let protos = ["proto/math.proto"];

    let protos_inc = ["proto"];

    prost_build::compile_protos(&protos, &protos_inc).unwrap();
}
