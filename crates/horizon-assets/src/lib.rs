//! Stub para horizon-assets (baseado em horizon-rust/horizon MIT).

pub mod palette {
    //! Paleta de cores (stub).

    pub struct Palette;

    impl Palette {
        pub fn new() -> Self { Self }
    }

    pub fn apply_palette() {}
}

pub mod model {
    //! Modelos de blocos/itens (stub).

    pub struct Model;
    pub struct Direction;
    pub struct CuboidModel;
    pub struct TextureSlot;

    impl Model {
        pub fn new() -> Self { Self }
    }

    impl Direction {
        pub fn new() -> Self { Self }
    }

    impl CuboidModel {
        pub fn new() -> Self { Self }
    }

    impl TextureSlot {
        pub fn new() -> Self { Self }
    }
}
