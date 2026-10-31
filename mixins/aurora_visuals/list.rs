fn aurora_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::aurora_visuals as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![["toggle".into(), "Toggle Aurora HUD".into(), "key.keyboard.o".into()]],
    });
}
