fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile_protos(
        &[
            "../../libs/proto/health.proto",
            "../../libs/proto/product.proto",
        ],
        &["../../libs/proto"],
    )?;
    Ok(())
}
