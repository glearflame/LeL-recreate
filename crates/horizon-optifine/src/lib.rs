//! Stub para horizon-optifine (baseado em horizon-rust/horizon MIT).

pub mod cem {
    //! Connected Emissive Models (stub).

    pub struct Cem;

    pub mod files {
        //! Arquivos CEM (stub).

        pub struct Files;
        pub struct atlas_name;
        pub struct jem_files;

        pub fn load_jem() {}

        impl Files {
            pub fn new() -> Self { Self }
        }

        impl atlas_name {
            pub fn new() -> Self { Self }
        }

        impl jem_files {
            pub fn new() -> Self { Self }
        }
    }

    pub mod model {
        //! Modelo CEM (stub).

        pub struct Model;
        pub struct Bone;
        pub struct EntityModel;

        impl Model {
            pub fn new() -> Self { Self }
        }

        impl Bone {
            pub fn new() -> Self { Self }
        }

        impl EntityModel {
            pub fn new() -> Self { Self }
        }
    }

    pub mod json {
        //! JSON CEM (stub).

        pub struct Json;

        impl Json {
            pub fn new() -> Self { Self }
        }
    }

    impl Cem {
        pub fn new() -> Self { Self }
    }
}

pub mod cit {
    //! Connected Textures (stub).

    pub struct Cit;
    pub struct Hand;
    pub struct Kind;
    pub struct Rule;
    pub struct Global;
    pub struct Method;
    pub struct ComponentTest;
    pub struct Source;
    pub struct ValueTest;

    pub fn connect() {}

    impl Cit {
        pub fn new() -> Self { Self }
    }

    impl Hand {
        pub fn new() -> Self { Self }
    }

    impl Kind {
        pub fn new() -> Self { Self }
    }

    impl Rule {
        pub fn new() -> Self { Self }
    }

    impl Global {
        pub fn new() -> Self { Self }
    }

    impl Method {
        pub fn new() -> Self { Self }
    }

    impl ComponentTest {
        pub fn new() -> Self { Self }
    }

    impl Source {
        pub fn new() -> Self { Self }
    }

    impl ValueTest {
        pub fn new() -> Self { Self }
    }
}

pub mod ctm {
    //! Connected Textures Models (stub).

    pub struct Ctm;
    pub struct Connect;
    pub struct Method;
    pub struct Orient;
    pub struct Rule;
    pub struct Tile;

    pub fn connect() {}

    impl Ctm {
        pub fn new() -> Self { Self }
    }

    impl Connect {
        pub fn new() -> Self { Self }
    }

    impl Method {
        pub fn new() -> Self { Self }
    }

    impl Orient {
        pub fn new() -> Self { Self }
    }

    impl Rule {
        pub fn new() -> Self { Self }
    }

    impl Tile {
        pub fn new() -> Self { Self }
    }
}

pub struct Properties;

impl Properties {
    pub fn new() -> Self { Self }
}

pub fn properties_files_in() {}

pub mod sky {
    //! Sky (stub).

    pub struct Sky;

    impl Sky {
        pub fn new() -> Self { Self }
    }
}
