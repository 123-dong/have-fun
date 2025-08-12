use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
};

const GENERATED_DIR: &str = "generated";
const DESCRIPTOR_FILE: &str = "all_descriptor.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let generated_path = proto_root.join(GENERATED_DIR);

    fs::create_dir_all(&generated_path)?;

    let proto_files = collect_proto_files(&proto_root);

    let proto_paths: Vec<String> = proto_files
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();

    println!("Found proto files: {:?}", proto_paths);

    tonic_build::configure()
        .file_descriptor_set_path(generated_path.join(DESCRIPTOR_FILE))
        .build_server(true)
        .build_client(false)
        .out_dir(&generated_path)
        .compile(&proto_paths, &[proto_root.to_string_lossy().as_ref()])?;

    Ok(())
}

fn collect_proto_files(dir: &Path) -> BTreeSet<PathBuf> {
    let mut result = BTreeSet::new();
    collect_proto_files_impl(dir, &mut result);
    result
}

fn collect_proto_files_impl(dir: &Path, result: &mut BTreeSet<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                if path.file_name().and_then(|n| n.to_str()) == Some(GENERATED_DIR) {
                    continue;
                }
                collect_proto_files_impl(&path, result);
            } else if path.extension().and_then(|e| e.to_str()) == Some("proto") {
                result.insert(path);
            }
        }
    }
}
