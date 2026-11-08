//! Executável do cliente Horizon.

use horizon_client::Game;

fn main() {
    println!("=== Horizon (recriação do zero) ===");
    println!("Baseado na estrutura do horizon-rust/horizon (MIT)");
    println!("Partes de terceiros excluídas: Discord, site, patrocínio, assets Mojang/Microsoft.\n");

    let mut game = Game::new(Default::default());
    let mut ticks = 0;
    while game.is_running() {
        game.tick();
        game.render();
        ticks += 1;
        println!(
            "tick {} | pos=({}, {}, {}) | frame={}",
            ticks, game.player_pos.x, game.player_pos.y, game.player_pos.z, game.renderer.frames_rendered
        );
    }

    println!("\nJogo encerrado após {} ticks. Frames renderizados: {}", ticks, game.renderer.frames_rendered);
}
