fn main() {
    protobuf_codegen::CodeGen::new()
        .include("src/routeguide")
        .inputs(["routeguide.proto"])
        .output_dir("generated")
        .compile_only()
        .unwrap();

    // NOTE: If you were to generate code yourself, you would use the
    // command below using tonic_protobuf_build's CodeGen.
    // tonic_protobuf_build::CodeGen::new()
    //     .include("src/routeguide")
    //     .inputs(["routeguide.proto"])
    //     .output_dir("generated")
    //     .compile_only()
    //     .unwrap();
}

