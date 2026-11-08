//! Stub para horizon-items (baseado em horizon-rust/horizon MIT).

pub mod blocks {
    //! Blocos (stub).

    pub struct Block;
    pub struct BlockFacts;
    pub const MAP_COLORS: &[u8] = &[];
    pub struct RenderShape;

    impl Block {
        pub fn new() -> Self { Self }
    }

    impl BlockFacts {
        pub fn new() -> Self { Self }
    }

    impl RenderShape {
        pub fn new() -> Self { Self }
    }
}

pub mod banner_layer {
    //! Camadas de banner (stub).

    pub struct BannerLayer;

    impl BannerLayer {
        pub fn new() -> Self { Self }
    }
}

pub mod items {
    //! Itens (stub).

    pub struct Item;
    pub struct ItemStack;

    impl Item {
        pub fn new() -> Self { Self }
    }

    impl ItemStack {
        pub fn new() -> Self { Self }
    }
}

pub mod item_data {
    //! Dados de item (stub).

    pub struct ItemData;

    pub fn canon_tag() {}

    pub fn encode_components() {}

    impl ItemData {
        pub fn new() -> Self { Self }
    }
}
