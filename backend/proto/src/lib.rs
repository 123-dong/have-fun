pub mod user {
    include!("../generated/user.v1.rs");
    pub(crate) const FILE_DESCRIPTOR_PATH: &[u8] =
        include_bytes!("../generated/all_descriptor.bin");
}
