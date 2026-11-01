fn aurora_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::aurora_visuals as m;
    // In singleplayer Horizon ticks both the client Game and its integrated
    // server Game in one process. Server ticks must NEVER clear the shared
    // client GPU snapshot, or ribbons restart between client frames.
    if !game.native_window() {
        static DIAG: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        if *DIAG.get_or_init(|| std::env::var("AURORA_RIBBON_DIAG").as_deref() == Ok("1")) {
            static LOGGED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if !LOGGED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                eprintln!("[aurora/ribbon] role=integrated_server ignored=true reason=no_native_window");
            }
        }
        return;
    }
    // Only the window/client Game owns activation and teardown of its visuals.
    if !game.native_on(m::ID) {
        horizon_client_render::gfx::native_render::aurora_gpu::inactive();
        return;
    }
    if let Some(player) = game.native_player() {
        m::sample_position(player.dimension, player.x, player.z);
        m::trail_sample(player.dimension, [player.x, player.y + 0.2, player.z]);
        let s = m::settings(|key| game.native_setting(m::ID, key));
        // Only a small copy of player state crosses the gameplay/render boundary.
        horizon_client_render::gfx::native_render::aurora_gpu::publish(
            player.dimension, Some([player.x, player.y + 0.15, player.z]),
            s.ribbon_enabled, s.ribbon_style, s.ribbon_demo);
        let burst = m::take_particle_burst();
        if s.particles {
            for fx in m::particle_emissions(
                player.dimension, [player.x, player.y, player.z], burst, s) {
                game.native_flame_particle(fx.pos, fx.soul);
            }
        }
        m::sample_frame(game.native_frame_rate().0);
    } else {
        m::reset_position();
        horizon_client_render::gfx::native_render::aurora_gpu::inactive();
    }
}
