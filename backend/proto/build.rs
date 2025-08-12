use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const OUT_DIR: &str = "generated";
    const FILE_DESCRIPTOR_SET: &str = "all_descriptor.bin";

    std::fs::create_dir_all(OUT_DIR)?;

    tonic_build::configure()
        .file_descriptor_set_path(Path::new(OUT_DIR).join(FILE_DESCRIPTOR_SET))
        .build_server(false)
        .build_client(false)
        .out_dir(OUT_DIR)
        .compile(&["user/v1/user.proto"], &["user"])?;

    Ok(())
}
