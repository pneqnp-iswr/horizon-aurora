fn aurora_mods(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: crate::game::aurora::MOD_ID.to_string(),
        title: "Horizon Aurora 0.1.0".to_string(),
        description: "Visual HUD suite with theme switching and click telemetry".to_string(),
        keys: vec![
            [
                "cycle_theme".to_string(),
                "Cycle theme".to_string(),
                "key.keyboard.f7".to_string(),
            ],
            [
                "reduced_motion".to_string(),
                "Toggle reduced motion".to_string(),
                "key.keyboard.f8".to_string(),
            ],
        ],
    });
}
