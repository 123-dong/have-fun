pub mod user {
    include!("../generated/user.v1.rs");
    pub(crate) const FILE_DESCRIPTOR_SET_PATH: &[u8] =
        tonic::include_file_descriptor_set!("all_descriptor");
}
