fn aurora_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::aurora_visuals as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_window() { return; }
    let s = m::settings(|key| game.native_setting(m::ID, key));
    if !s.shown { return; }
    let Some(player) = game.native_player() else { return; };
    let corner = [Corner::TopLeft, Corner::TopRight, Corner::BottomLeft, Corner::BottomRight][s.corner];
    if !game.native_corner_shown(corner) { return; }
    let (fps, _, low_fps) = game.native_frame_rate();
    let reading = m::Reading {
        fps, low_fps,
        clicks: game.native_clicks(),
        bps: m::movement_bps(),
        position: Some((player.x, player.y, player.z)),
        keys: ["key.forward", "key.left", "key.back", "key.right"]
            .map(|name| game.native_mapping(name).is_some_and(|v| v.0)),
    };
    let view = m::panel(s, reading, m::frame_history(), m::input_pulse(),
        &|text| game.native_text_width(text));
    let (x, y) = corners.place(corner, view.width, view.height);
    for shape in view.shapes {
        match shape {
            m::Shape::Fill([dx, dy, w, h], argb) => game.native_fill(batch, [x + dx, y + dy, w, h], argb),
            m::Shape::Text(text, [dx, dy], color, shadow) => game.native_text(batch, &text, [x + dx, y + dy], color, shadow),
        }
    }
}
