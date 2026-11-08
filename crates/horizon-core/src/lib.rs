//! Núcleo do Horizon: posições de bloco/chunk/seção, matemática,
//! gerador de números aleatórios xoroshiro e geometria de colisão.
//!
//! Recriação do zero — partes de 100% de autoria do horizon-rust/horizon.
//! Não contém código de terceiros.

use core::ops::{Add, AddAssign, Sub, SubAssign};

/// Posição de um bloco no mundo (inteiros, em blocos).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const ORIGIN: Self = Self { x: 0, y: 0, z: 0 };

    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn squared_distance(self, other: Self) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

impl Add for BlockPos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for BlockPos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for BlockPos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for BlockPos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Posição de um chunk (cada chunk tem 16x16 blocos na horizontal, 256 na vertical).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub const ORIGIN: Self = Self { x: 0, z: 0 };

    /// Converte uma posição de bloco para a posição do chunk que a contém.
    pub fn from_block_pos(pos: BlockPos) -> Self {
        Self {
            x: pos.x.div_euclid(16),
            z: pos.z.div_euclid(16),
        }
    }
}

/// Posição de uma seção de chunk (16x16x16 blocos).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SectionPos {
    pub chunk: ChunkPos,
    pub y: i32,
}

impl SectionPos {
    pub fn new(chunk: ChunkPos, y: i32) -> Self {
        Self { chunk, y }
    }
}

/// Caixa delimitadora alinhada aos eixos (AABB) para colisão, em precisão dupla.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}

impl Aabb {
    pub const fn new(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Self {
        Self { min_x, min_y, min_z, max_x, max_y, max_z }
    }

    pub fn intersects(self, other: Self) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
            && self.min_z < other.max_z
            && self.max_z > other.min_z
    }

    pub fn move_by(self, dx: f64, dy: f64, dz: f64) -> Self {
        Self {
            min_x: self.min_x + dx,
            min_y: self.min_y + dy,
            min_z: self.min_z + dz,
            max_x: self.max_x + dx,
            max_y: self.max_y + dy,
            max_z: self.max_z + dz,
        }
    }

    pub fn inflate(self, dx: f64, dy: f64, dz: f64) -> Self {
        Self {
            min_x: self.min_x - dx,
            min_y: self.min_y - dy,
            min_z: self.min_z - dz,
            max_x: self.max_x + dx,
            max_y: self.max_y + dy,
            max_z: self.max_z + dz,
        }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 0.0,
            max_y: 0.0,
            max_z: 0.0,
        }
    }
}

/// Gerador de números aleatórios xoroshiro128++ (usado pelo Horizon para worldgen).
#[derive(Debug, Clone, Copy)]
pub struct Xoroshiro128PlusPlus {
    s0: u64,
    s1: u64,
}

impl Xoroshiro128PlusPlus {
    pub fn new(seed: u64) -> Self {
        let s0 = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(0xBF58_476D_1CE4_35B9);
        let s1 = seed.wrapping_mul(0xC2B2_AE3D_27D4_44F5).wrapping_add(0x94D0_49BB_1331_11EB);
        Self { s0, s1 }
    }

    fn rotl64(x: u64, k: u32) -> u64 {
        x.rotate_left(k)
    }

    pub fn next_u64(&mut self) -> u64 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.s0 = Self::rotl64(s0, 55) ^ s1 ^ (s1 << 14);
        self.s1 = Self::rotl64(s1, 36);

        result
    }

    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Número inteiro em [0, bound).
    pub fn next_int(&mut self, bound: i32) -> i32 {
        (self.next_u64() as u128 * bound as u128 >> 64) as i32
    }
}

impl Default for Xoroshiro128PlusPlus {
    fn default() -> Self {
        Self::new(0)
    }
}

// Módulos stub necessários pelo client_render (horizon-rust/horizon).
pub mod geometry {
    //! Geometria de colisão e renderização (stub).

    pub struct Aabb {
        pub min_x: f64,
        pub min_y: f64,
        pub min_z: f64,
        pub max_x: f64,
        pub max_y: f64,
        pub max_z: f64,
    }

    impl Aabb {
        pub const fn new(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Self {
            Self { min_x, min_y, min_z, max_x, max_y, max_z }
        }
    }

    /// Direção cardinal (stub).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Direction {
        North,
        South,
        East,
        West,
        Up,
        Down,
    }

    /// Funções trigonométricas (stub).
    pub mod trig {
        pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
        pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

        pub struct Mth;

        impl Mth {
            pub fn sin(_x: f64) -> f64 { 0.0 }
            pub fn cos(_x: f64) -> f64 { 1.0 }
            pub fn tan(_x: f64) -> f64 { 0.0 }
            pub fn asin(_x: f64) -> f64 { 0.0 }
            pub fn acos(_x: f64) -> f64 { 0.0 }
            pub fn atan(_x: f64) -> f64 { 0.0 }
            pub fn atan2(_y: f64, _x: f64) -> f64 { 0.0 }
        }
    }

    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Vec3 {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }
    }

    pub struct VoxelShape;

    impl VoxelShape {
        pub fn empty() -> Self { Self }
    }

    pub struct border;

    pub struct ShapeOp;
}

pub mod noise {
    //! Geradores de ruído para worldgen (stub).

    pub struct SimplexNoise;

    impl SimplexNoise {
        pub fn new() -> Self { Self }
        pub fn sample(&self, _x: f64, _z: f64) -> f64 { 0.0 }
    }

    pub struct NoiseSampler;

    impl NoiseSampler {
        pub fn new() -> Self { Self }
        pub fn sample(&self, _x: f64, _z: f64) -> f64 { 0.0 }
    }
}

pub mod position {
    //! Tipos de posição para blocos e chunks (reexporta do crate raiz).

    pub use super::{BlockPos, ChunkPos, SectionPos};
}

pub mod random {
    //! Geração de números aleatórios (reexporta do crate raiz).

    pub use super::Xoroshiro128PlusPlus as Random;

    /// Gerador legado de números aleatórios (stub).
    pub struct LegacyRandom {
        seed: u64,
    }

    impl LegacyRandom {
        pub fn new(seed: u64) -> Self { Self { seed } }
        pub fn next_float(&mut self) -> f64 { 0.0 }
    }

    /// Fonte de números aleatórios (stub).
    pub struct RandomSource;

    impl RandomSource {
        pub fn new() -> Self { Self }
    }
}

// Módulo paletted (stub para suporte a blocos paletizados).
pub mod paletted {
    //! Containers de estado de bloco paletizado (stub).

    pub struct Strategy;

    pub struct PalettedContainer<T> {
        data: Vec<T>,
    }

    impl<T: Clone + Default> PalettedContainer<T> {
        pub fn new(default: T, size: usize) -> Self {
            Self { data: vec![default; size] }
        }

        pub fn get(&self, _index: usize) -> Option<&T> {
            None
        }

        pub fn set(&mut self, _index: usize, _value: T) {}
    }
}

// Módulo fast_hash (stub).
pub mod fast_hash {
    //! Funções de hash rápido (stub).

    pub fn hash(_data: &[u8]) -> u64 { 0 }
}

// Módulo prepared_files (stub).
pub mod prepared_files {
    //! Arquivos preparados (stub).

    pub struct PreparedFiles;

    impl PreparedFiles {
        pub fn new() -> Self { Self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_pos_distance() {
        let a = BlockPos::new(0, 0, 0);
        let b = BlockPos::new(3, 4, 0);
        assert_eq!(a.squared_distance(b), 25);
    }

    #[test]
    fn chunk_from_block() {
        assert_eq!(ChunkPos::from_block_pos(BlockPos::new(0, 0, 0)), ChunkPos::ORIGIN);
        assert_eq!(ChunkPos::from_block_pos(BlockPos::new(16, 0, 0)), ChunkPos::new(1, 0));
        assert_eq!(ChunkPos::from_block_pos(BlockPos::new(-1, 0, 0)), ChunkPos::new(-1, 0));
    }

    #[test]
    fn aabb_intersects() {
        let a = Aabb::new(0.0, 0.0, 0.0, 2.0, 2.0, 2.0);
        let b = Aabb::new(1.0, 1.0, 1.0, 3.0, 3.0, 3.0);
        assert!(a.intersects(b));
        let c = Aabb::new(5.0, 5.0, 5.0, 7.0, 7.0, 7.0);
        assert!(!a.intersects(c));
    }

    #[test]
    fn xoroshiro_deterministic() {
        let mut a = Xoroshiro128PlusPlus::new(12345);
        let mut b = Xoroshiro128PlusPlus::new(12345);
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn xoroshiro_not_degenerate() {
        let mut rng = Xoroshiro128PlusPlus::new(0);
        let mut seen_zero = 0u32;
        for _ in 0..1000 {
            if rng.next_u64() == 0 {
                seen_zero += 1;
            }
        }
        assert!(seen_zero < 1000);
    }
}
