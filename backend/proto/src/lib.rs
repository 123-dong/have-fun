pub mod v1 {
    pub const DESCRIPTOR_SET: &[u8] = include_bytes!("../generated/all_descriptor.bin");

    pub mod user {
        include!("../generated/user.v1.rs");
    }
    pub mod product {
        include!("../generated/product.v1.rs");
    }

    pub mod chat {
        include!("../generated/chat.v1.rs");
    }
}
