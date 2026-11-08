//! Stub para horizon-world (baseado em horizon-rust/horizon MIT).

pub const DATA_VERSION: i32 = 2606;

pub mod worldgen {
    //! Geração de mundo (stub).

    pub mod tags {
        //! Tags de worldgen (stub).

        pub struct Tags;
        pub struct TagDefinitions;

        impl Tags {
            pub fn new() -> Self { Self }
        }

        impl TagDefinitions {
            pub fn new() -> Self { Self }
        }
    }

    pub mod terrain {
        //! Geração de terreno (stub).

        pub struct Terrain;
        pub struct TerrainBlockStates;

        impl Terrain {
            pub fn new() -> Self { Self }
        }

        impl TerrainBlockStates {
            pub fn new() -> Self { Self }
        }
    }

    pub mod biome_temperature {
        //! Temperatura de bioma (stub).

        pub struct BiomeTemperature;
        pub struct BiomeTemperatureSettings;
        pub struct TemperatureModifier;

        impl BiomeTemperature {
            pub fn new() -> Self { Self }
        }

        impl BiomeTemperatureSettings {
            pub fn new() -> Self { Self }
        }

        impl TemperatureModifier {
            pub fn new() -> Self { Self }
        }
    }

    pub mod overrides {
        //! Overrides de worldgen (stub).

        pub struct Overrides;

        impl Overrides {
            pub fn new() -> Self { Self }
        }
    }

    pub struct ChunkGenerator;

    impl ChunkGenerator {
        pub fn new() -> Self { Self }
    }
}

pub mod heightmap {
    //! Mapas de altura (stub).

    pub struct Heightmap;
    pub struct HeightmapState;

    impl Heightmap {
        pub fn new() -> Self { Self }
    }

    impl HeightmapState {
        pub fn new() -> Self { Self }
    }
}

pub mod registry {
    //! Registro de blocos e itens (stub).

    pub struct Registry;
    pub struct BlockRegistry;
    pub struct BlockStateId;
    pub struct BlockTableFile;

    impl Registry {
        pub fn new() -> Self { Self }
    }

    impl BlockRegistry {
        pub fn new() -> Self { Self }
    }

    impl BlockStateId {
        pub fn new() -> Self { Self }
    }

    impl BlockTableFile {
        pub fn new() -> Self { Self }
    }
}

pub mod block_render {
    //! Renderização de blocos (stub).

    pub struct BlockRender;
    pub struct FaceVisibility;

    impl BlockRender {
        pub fn new() -> Self { Self }
    }

    impl FaceVisibility {
        pub fn new() -> Self { Self }
    }
}

pub mod block_collision {
    //! Colisão de blocos (stub).

    pub struct BlockCollision;
    pub struct BlockCollisionRegistry;
    pub struct DynamicCollisionWorld;
    pub struct ShulkerCollisionState;

    impl BlockCollision {
        pub fn new() -> Self { Self }
    }

    impl BlockCollisionRegistry {
        pub fn new() -> Self { Self }
    }

    impl DynamicCollisionWorld {
        pub fn new() -> Self { Self }
    }

    impl ShulkerCollisionState {
        pub fn new() -> Self { Self }
    }
}

pub mod block_collision_scan {
    //! Escaneamento de colisão (stub).

    pub struct BlockCollisionScan;
    pub struct BlockCollisionWorld;
    pub struct CollisionBlock;

    impl BlockCollisionScan {
        pub fn new() -> Self { Self }
    }

    impl BlockCollisionWorld {
        pub fn new() -> Self { Self }
    }

    impl CollisionBlock {
        pub fn new() -> Self { Self }
    }
}

pub mod collision {
    //! Colisão (stub).

    pub struct Collision;
    pub struct CachedCollisionRegistry;

    impl Collision {
        pub fn new() -> Self { Self }
    }

    impl CachedCollisionRegistry {
        pub fn new() -> Self { Self }
    }
}

pub mod collision_context {
    //! Contexto de colisão (stub).

    pub struct CollisionContext;
    pub struct CollisionEntity;
    pub struct CollisionItem;

    impl CollisionContext {
        pub fn new() -> Self { Self }
    }

    impl CollisionEntity {
        pub fn new() -> Self { Self }
    }

    impl CollisionItem {
        pub fn new() -> Self { Self }
    }
}

pub mod dynamic_support {
    //! Suporte dinâmico (stub).

    pub struct DynamicSupport;
    pub struct DynamicSupportWorld;

    impl DynamicSupport {
        pub fn new() -> Self { Self }
    }

    impl DynamicSupportWorld {
        pub fn new() -> Self { Self }
    }
}

pub mod entity_obstruction {
    //! Obstrução de entidades (stub).

    pub struct EntityObstruction;
    pub struct EntityIdentity;

    impl EntityObstruction {
        pub fn new() -> Self { Self }
    }

    impl EntityIdentity {
        pub fn new() -> Self { Self }
    }
}

pub mod fluid {
    //! Fluidos (stub).

    pub struct Fluid;
    pub struct FluidState;
    pub struct FluidKind;

    impl Fluid {
        pub fn new() -> Self { Self }
    }

    impl FluidState {
        pub fn new() -> Self { Self }
    }

    impl FluidKind {
        pub fn new() -> Self { Self }
    }
}

pub mod liquid_collision {
    //! Colisão líquida (stub).

    pub struct LiquidCollision;
    pub struct LiquidCollisionWorld;

    impl LiquidCollision {
        pub fn new() -> Self { Self }
    }

    impl LiquidCollisionWorld {
        pub fn new() -> Self { Self }
    }
}

pub mod piston_shape {
    //! Forma de pistão (stub).

    pub struct PistonShape;
    pub struct PistonCollisionWorld;

    impl PistonShape {
        pub fn new() -> Self { Self }
    }

    impl PistonCollisionWorld {
        pub fn new() -> Self { Self }
    }
}

pub mod powder_snow_collision {
    //! Colisão de neve em pó (stub).

    pub struct PowderSnowCollision;
    pub struct PowderSnowEntity;
    pub struct PowderSnowWorld;

    impl PowderSnowCollision {
        pub fn new() -> Self { Self }
    }

    impl PowderSnowEntity {
        pub fn new() -> Self { Self }
    }

    impl PowderSnowWorld {
        pub fn new() -> Self { Self }
    }
}

pub mod loot {
    //! Sistema de loot (stub).

    pub struct LootTable;
    pub struct PackOverlay;

    impl LootTable {
        pub fn new() -> Self { Self }
    }

    impl PackOverlay {
        pub fn new() -> Self { Self }
    }
}

pub mod nbt {
    //! NBT (Named Binary Tag) (stub).

    pub struct Nbt;
    pub struct Tag;
    pub struct Compound;
    pub struct JavaString;

    pub fn parse_snbt() {}

    impl Nbt {
        pub fn new() -> Self { Self }
    }

    impl Tag {
        pub fn new() -> Self { Self }
    }

    impl Compound {
        pub fn new() -> Self { Self }
    }

    impl JavaString {
        pub fn new() -> Self { Self }
    }
}

pub mod world_movement {
    //! Movimento de mundo (stub).

    pub struct WorldMovement;
    pub struct CollisionCheckRequest;
    pub struct EntityMovementWorld;
    pub struct MovementRequest;

    pub fn no_world_collision() {}
    pub fn resolve_world_movement() {}

    impl WorldMovement {
        pub fn new() -> Self { Self }
    }

    impl CollisionCheckRequest {
        pub fn new() -> Self { Self }
    }

    impl EntityMovementWorld {
        pub fn new() -> Self { Self }
    }

    impl MovementRequest {
        pub fn new() -> Self { Self }
    }
}

pub mod environment_attributes {
    //! Atributos de ambiente (stub).

    pub struct EnvironmentAttributes;

    impl EnvironmentAttributes {
        pub fn new() -> Self { Self }
    }
}

pub mod environment_system {
    //! Sistema de ambiente (stub).

    pub struct EnvironmentSystem;

    impl EnvironmentSystem {
        pub fn new() -> Self { Self }
    }
}

pub mod environment_timeline {
    //! Linha do tempo de ambiente (stub).

    pub struct EnvironmentTimeline;

    impl EnvironmentTimeline {
        pub fn new() -> Self { Self }
    }
}

pub mod block_state_properties {
    //! Propriedades de estado de bloco (stub).

    pub struct BlockStateProperties;

    impl BlockStateProperties {
        pub fn new() -> Self { Self }
    }
}

pub mod level {
    //! Nível/mundo (stub).

    pub mod sculk {
        //! Sculk (stub).

        pub struct Sculk;

        impl Sculk {
            pub fn new() -> Self { Self }
        }
    }

    pub struct Level;

    impl Level {
        pub fn new() -> Self { Self }
    }
}
