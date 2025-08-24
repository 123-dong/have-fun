pub mod config;
pub mod database;
pub mod errors;
pub mod models;
pub mod utils;

#[macro_export]
macro_rules! init_reflection {
    ($desc_set:expr) => {{
        tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set($desc_set)
            .build_v1()
    }};
}
