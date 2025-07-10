fn main() {
    // tonic_build::compile_protos("src/routeguide/routeguide.proto")
    //     .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    tonic_protobuf_build::CodeGen::new()
        .include("src/routeguide")
        .inputs(["routeguide.proto"])
        .generate_and_compile()
        .unwrap();
}
