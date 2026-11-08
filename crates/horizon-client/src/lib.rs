//! Loop de jogo do Horizon (cliente).
//!
//! Recriação do zero — partes de 100% de autoria do horizon-rust/horizon.
//! Não contém código de terceiros (sem janela gráfica real, sem assets, sem Discord).

use horizon_assets::{AssetManager, ResourcePack, ResourceLocation};
use horizon_core::{BlockPos, Xoroshiro128PlusPlus};
use horizon_protocol::{Packet, PacketId};
use horizon_render::{Renderer, StubRenderer};
use horizon_world::{BlockId, World};
use std::env;
use std::path::PathBuf;

/// Resolve o diretório de assets relativo ao executável do jogo.
/// Funciona tanto em desenvolvimento (`target/release/`) quanto após instalação.
fn assets_dir() -> PathBuf {
    let exe = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    match exe.parent() {
        Some(dir) => dir.join("assets"),
        None => PathBuf::from("assets"),
    }
}

/// Configuração do jogo.
pub struct GameConfig {
    pub max_frames: u64,
    pub render_distance_chunks: i32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            max_frames: 10,
            render_distance_chunks: 8,
        }
    }
}

/// O jogo do Horizon.
pub struct Game {
    pub config: GameConfig,
    pub world: World,
    pub renderer: StubRenderer,
    pub rng: Xoroshiro128PlusPlus,
    pub frames: u64,
    pub player_pos: BlockPos,
    pub running: bool,
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let mut renderer = StubRenderer::new();
        renderer.init();

        let mut world = World::new();
        // Gera um pequeno terreno simples ao redor do spawn (código próprio).
        let seed = 0xC0FFEE;
        let mut rng = Xoroshiro128PlusPlus::new(seed);
        for x in -16..16 {
            for z in -16..16 {
                let height = 60 + rng.next_int(6);
                for y in 0..=height {
                    let id = if y == height { BlockId::GRASS_BLOCK } else { BlockId::DIRT };
                    let below = if y == 0 { BlockId::STONE } else { id };
                    world.set_block(BlockPos::new(x, y, z), below, Default::default());
                }
                // Toque de pedra abaixo.
                world.set_block(BlockPos::new(x, height + 1, z), BlockId::AIR, Default::default());
            }
        }
        world.spawn = BlockPos::new(0, 64, 0);
        let spawn = world.spawn;

        let mut assets = AssetManager::new();
        assets.add_pack(ResourcePack::new("default", assets_dir()));

        Self {
            config,
            world,
            renderer,
            rng,
            frames: 0,
            player_pos: spawn,
            running: true,
        }
    }

    /// Executa um tick do jogo.
    pub fn tick(&mut self) {
        // Move o jogador levemente (simulação própria).
        self.player_pos.x += 1;
        self.player_pos.y = self.world.get_block(self.player_pos).0 as i32 % 4;
        self.player_pos.z += 1;

        // Envia um pacote de posição simulado (protocolo próprio).
        let _packet = Packet::new(PacketId::PLAY_POSITION, {
            let mut b = Vec::new();
            b.extend_from_slice(&self.player_pos.x.to_le_bytes());
            b.extend_from_slice(&self.player_pos.y.to_le_bytes());
            b.extend_from_slice(&self.player_pos.z.to_le_bytes());
            b
        });

        // Resolve um recurso do pack (assets próprios/vazios).
        let _loc = ResourceLocation::from_str("minecraft:stone").unwrap();
        let _ = self.assets_read(&_loc);

        self.frames += 1;
        if self.frames >= self.config.max_frames {
            self.running = false;
        }
    }

    /// Renderiza um frame.
    pub fn render(&mut self) {
        let result = self.renderer.render_frame();
        if result == horizon_render::RenderResult::Lost {
            self.running = false;
        }
        // Desenha a caixa do jogador (debug).
        let aabb = horizon_core::Aabb::new(
            self.player_pos.x as f64,
            self.player_pos.y as f64,
            self.player_pos.z as f64,
            (self.player_pos.x + 1) as f64,
            (self.player_pos.y + 2) as f64,
            (self.player_pos.z + 1) as f64,
        );
        self.renderer.draw_aabb(aabb);
    }

    /// Lê um recurso do gerenciador de assets (stub).
    fn assets_read(&self, loc: &ResourceLocation) -> Option<String> {
        // Sem assets reais; apenas registra a tentativa.
        let _ = loc;
        None
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.renderer.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_runs_and_stops() {
        let mut game = Game::new(GameConfig { max_frames: 5, ..Default::default() });
        let mut ticks = 0;
        while game.is_running() {
            game.tick();
            game.render();
            ticks += 1;
            assert!(ticks < 100, "loop infinito");
        }
        assert_eq!(ticks, 5);
        assert_eq!(game.renderer.frames_rendered, 5);
    }

    #[test]
    fn world_generation_deterministic() {
        let a = Game::new(Default::default());
        let b = Game::new(Default::default());
        // Mesma semente -> mesmo spawn e mesmo bloco no spawn.
        assert_eq!(a.world.spawn, b.world.spawn);
        assert_eq!(
            a.world.get_block(a.world.spawn),
            b.world.get_block(b.world.spawn)
        );
    }
}
