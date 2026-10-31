fn aurora_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::aurora_visuals as m;
    if mod_id != m::ID || button >= 22 { return; }
    let mut s = m::settings(|key| game.native_setting(m::ID, key));
    s.click(button);
    m::set_settings(s);
    for (key, value) in s.stored() { game.native_set_setting(m::ID, key, &value); }
}
