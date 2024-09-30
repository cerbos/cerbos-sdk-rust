fn main() -> Result<(), std::io::Error> {
    tonic_build::configure()
        .out_dir("src/genpb")
        .build_server(false)
        .compile_protos(&["proto/defs/cerbos/svc/v1/svc.proto"], &["proto/defs/"])
}
