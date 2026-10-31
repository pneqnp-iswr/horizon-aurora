fn aurora_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::aurora_visuals as m;
    if *ran || !game.native_on(m::ID) || !game.native_window() { return; }
    let mut words = command.split_whitespace();
    if words.next() != Some("aurora") { return; }
    *ran = true;
    let first = words.next();
    let second = words.next();
    if words.next().is_some() { game.native_chat("Aurora: too many arguments. Use /aurora help"); return; }
    let mut s = m::settings(|key| game.native_setting(m::ID, key));
    match (first, second) {
        (None, None) | (Some("help"), None) => {
            game.native_chat("/aurora toggle | preset combat|minimal|streamer | theme violet|cyan|amber|rose|mint|electric|scarlet");
            game.native_chat("/aurora graph|feedback|ambient|compact|motion|trail|flare|adaptive|particles|crown on|off");
            game.native_chat("/aurora fire soul|flame | burst ring|burst|spiral | path off|line|twin|helix | reset");
            return;
        }
        (Some("toggle"), None) => s.shown = !s.shown,
        (Some("reset"), None) => s = m::Settings::default(),
        (Some("preset"), Some(value)) => {
            if let Some(index) = m::PRESETS.iter().position(|p| p.0 == value) { s.apply_preset(index); }
            else { game.native_chat("Aurora: unknown preset."); return; }
        }
        (Some("burst"), Some(value)) => {
            if let Some(i) = m::BURST_STYLES.iter().position(|v| *v == value) { s.burst_style = i; }
            else { game.native_chat("Aurora: burst ring|burst|spiral"); return; }
        }
        (Some("path"), Some(value)) => {
            if let Some(i) = m::TRAIL_STYLES.iter().position(|v| *v == value) { s.trail_style = i; }
            else { game.native_chat("Aurora: path off|line|twin|helix"); return; }
        }
        (Some("fire"), Some(value)) => {
            match value {
                "soul" => s.soul_particles = true,
                "flame" => s.soul_particles = false,
                _ => { game.native_chat("Aurora: use fire soul or fire flame."); return; },
            }
        }
        (Some("theme"), Some(value)) => {
            if let Some(index) = m::THEMES.iter().position(|t| t.0 == value) { s.theme = index; }
            else { game.native_chat("Aurora: unknown theme."); return; }
        }
        (Some(field @ ("graph" | "feedback" | "ambient" | "compact" | "motion" | "trail" | "flare" | "adaptive" | "particles" | "crown")), Some(value)) => {
            let flag = match value { "on" => true, "off" => false, _ => {
                game.native_chat("Aurora: use on or off."); return;
            } };
            match field {
                "graph" => s.graph = flag,
                "feedback" => s.feedback = flag,
                "ambient" => s.ambient = flag,
                "compact" => s.compact = flag,
                "motion" => s.reduced_motion = !flag,
                "trail" => s.trail = flag,
                "flare" => s.click_flare = flag,
                "adaptive" => s.adaptive_quality = flag,
                "particles" => s.particles = flag,
                "crown" => s.particle_crown = flag,
                _ => unreachable!(),
            }
        }
        _ => { game.native_chat("Aurora: invalid command. Use /aurora help"); return; }
    }
    m::set_settings(s);
    for (key, value) in s.stored() { game.native_set_setting(m::ID, key, &value); }
    game.native_chat(&format!("Aurora {} | {} | HUD {}", m::PRESETS[s.preset].1, m::THEMES[s.theme].1,
        if s.shown { "ON" } else { "OFF" }));
}
