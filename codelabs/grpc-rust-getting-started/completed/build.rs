fn main() {
    tonic_build::configure()
        .compile_protos(&["routeguide/routeguide.proto"], &["proto"])
        .unwrap();
}

