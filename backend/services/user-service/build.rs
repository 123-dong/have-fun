fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile_protos(&["../../libs/proto/user.proto"], &["../../libs/proto"])?;
    Ok(())
}
