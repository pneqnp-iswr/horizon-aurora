fn aurora_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::aurora_visuals as m;
    if !game.native_window() || !game.native_on(m::ID) { return; }
    if let Some(player) = game.native_player() {
        m::sample_position(player.dimension, player.x, player.z);
        m::trail_sample(player.dimension, [player.x, player.y + 0.2, player.z]);
        m::sample_frame(game.native_frame_rate().0);
    } else {
        m::reset_position();
    }
}
