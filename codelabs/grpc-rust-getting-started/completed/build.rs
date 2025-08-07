fn main() {
    protobuf_codegen::CodeGen::new()
        .include("src/routeguide")
        .inputs(["routeguide.proto"])
        .output_dir("generated")
        .compile_only()
        .unwrap();
    // tonic_protobuf_build::CodeGen::new()
    //     .include("src/routeguide")
    //     .inputs(["routeguide.proto"])
    //     .output_dir("generated")
    //     .compile_only()
    //     .unwrap();
}

