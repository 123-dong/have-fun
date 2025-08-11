use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let path = "all_descriptor.bin";

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join(path))
        .build_server(false)
        .build_client(false)
        .out_dir("generated")
        .compile(&["user/v1/user.proto"], &["proto"])?;
    Ok(())
}
