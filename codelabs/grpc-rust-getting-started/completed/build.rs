fn main() {
    let proto = "src/routeguide/routeguide.proto";

    tonic_prost_build::compile_protos(proto).unwrap();
    tonic_protobuf_build::CodeGen::new()
        .include("src/routeguide")
        .inputs(["routeguide.proto"])
        .compile()
        .unwrap();
}


