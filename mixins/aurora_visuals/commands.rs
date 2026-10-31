// API 35 client command list, used for chat completion / slash-command discovery.
fn aurora_commands(game: &mut crate::game::Game<'_>, commands: &mut Vec<&'static str>) {
    use crate::game::aurora_visuals as m;
    if !game.native_window() || !game.native_on(m::ID) { return; }
    if !commands.contains(&"aurora") { commands.push("aurora"); }
}
