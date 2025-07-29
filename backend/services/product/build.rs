fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile_protos(
        &["../../shared/proto/health.proto"],
        &["../../shared/proto"],
    )?;
    Ok(())
}
