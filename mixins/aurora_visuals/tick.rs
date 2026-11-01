fn aurora_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::aurora_visuals as m;
    if !game.native_window() || !game.native_on(m::ID) {
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
