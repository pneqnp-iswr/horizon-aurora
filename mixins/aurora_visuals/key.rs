fn aurora_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::aurora_visuals as m;
    if !pressed || !game.native_window() || !game.native_playing() || !game.native_on(m::ID) { return; }
    // A physical mouse press is an INPUT animation, not a server-confirmed hit.
    let s = m::settings(|k| game.native_setting(m::ID, k));
    if s.feedback || s.click_flare {
        match key {
            "key.mouse.left" => m::mouse_press(true),
            "key.mouse.right" => m::mouse_press(false),
            _ => {}
        }
    }
    let toggle = game.native_mod_key(m::ID, "toggle", "key.keyboard.o");
    if key != toggle { return; }
    let mut s = s;
    s.shown = !s.shown;
    m::set_settings(s);
    game.native_set_setting(m::ID, "shown", if s.shown { "true" } else { "false" });
    game.native_chat(if s.shown { "Aurora HUD: ON" } else { "Aurora HUD: OFF" });
}
