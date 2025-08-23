pub const DESCRIPTOR_SET: &[u8] = include_bytes!("../generated/all_descriptor.bin");

pub mod user {
    pub mod v1 {
        include!("../generated/user.v1.rs");
    }
}
