fn aurora_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::aurora_visuals as m;
    if mod_id != m::ID { return; }
    buttons.extend(m::settings(|key| game.native_setting(m::ID, key)).labels());
}
