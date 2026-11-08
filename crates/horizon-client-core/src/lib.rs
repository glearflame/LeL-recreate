//! Stub para horizon-client-core (baseado em horizon-rust/horizon MIT).

pub mod pattern {
    //! Padrões (stub).

    pub struct Pattern;

    impl Pattern {
        pub fn new() -> Self { Self }
    }
}

pub mod particle_type {
    //! Tipos de partículas (stub).

    pub struct ParticleType;

    impl ParticleType {
        pub fn new() -> Self { Self }
    }
}

pub mod potions {
    //! Poções (stub).

    pub struct Potion;
    pub struct PotionDef;
    pub struct EffectDef;
    pub struct Category;

    pub const BASE_POTION_COLOR: i32 = 0;

    pub const EFFECTS: &[u8] = &[];
    pub const POTIONS: &[u8] = &[];

    pub fn all_effects() {}

    pub fn description_id() {}

    pub fn effect() {}

    pub fn is_mod_effect() {}

    pub fn mod_effects() {}

    pub fn namespaced() {}

    pub fn potion() {}

    pub fn potion_color() {}

    pub fn register_mod_effects() {}

    impl Potion {
        pub fn new() -> Self { Self }
    }

    impl PotionDef {
        pub fn new() -> Self { Self }
    }

    impl EffectDef {
        pub fn new() -> Self { Self }
    }

    impl Category {
        pub fn new() -> Self { Self }
    }
}

pub mod nbt_value {
    //! Valores NBT (stub).

    pub struct NbtValue;

    pub mod number {
        //! Números NBT (stub).

        pub struct Number;

        impl Number {
            pub fn new() -> Self { Self }
        }
    }

    pub mod text {
        //! Texto NBT (stub).

        pub struct Text;

        impl Text {
            pub fn new() -> Self { Self }
        }
    }

    impl NbtValue {
        pub fn new() -> Self { Self }
    }
}

pub mod scoreboard {
    //! Placar (stub).

    pub struct Scoreboard;

    impl Scoreboard {
        pub fn new() -> Self { Self }
    }

    pub fn comp_text() {}
}

pub mod text {
    //! Texto (stub).

    pub struct Text;

    pub fn name_text() {}

    impl Text {
        pub fn new() -> Self { Self }
    }
}

pub mod component {
    //! Componentes (stub).

    pub struct Component;

    impl Component {
        pub fn new() -> Self { Self }
    }
}

pub mod who {
    //! Quem (stub).

    pub struct Who;

    impl Who {
        pub fn new() -> Self { Self }
    }
}
