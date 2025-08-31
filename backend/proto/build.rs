use std::{env, fs, path::PathBuf};

const GENERATED_DIR: &str = "generated";
const DESCRIPTOR_FILE: &str = "all_descriptor.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let generated_path = proto_root.join(GENERATED_DIR);
    fs::create_dir_all(&generated_path)?;

    let proto_files = collect_proto_files(&proto_root, &generated_path);

    println!("Found proto files: {proto_files:?}");

    tonic_build::configure()
        .file_descriptor_set_path(generated_path.join(DESCRIPTOR_FILE))
        .build_server(true)
        .build_client(true)
        .out_dir(&generated_path)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&proto_files, &[proto_root.to_str().unwrap()])?;

    Ok(())
}

fn collect_proto_files(dir: &PathBuf, skip_dir: &PathBuf) -> Vec<String> {
    let mut protos = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path == *skip_dir {
                    continue;
                }
                protos.extend(collect_proto_files(&path, skip_dir));
            } else if path.extension().and_then(|e| e.to_str()) == Some("proto") {
                protos.push(path.to_str().unwrap().to_string());
            }
        }
    }
    protos
}
