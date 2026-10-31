// API 46: world-space lights requested through Horizon's native renderer.
// They are local shader lighting, not gameplay light or server packets.
fn aurora_visuals_world(
    game: &crate::game::Game<'_>,
    rims: &mut Vec<crate::game::native_hooks::NativeBlockRim>,
    lights: &mut Vec<crate::game::native_hooks::NativeColoredLight>,
) {
    use crate::game::aurora_visuals as m;
    let _ = rims;
    if !game.native_window() || !game.native_on(m::ID) { return; }
    let s = m::settings(|key| game.native_setting(m::ID, key));
    if !(s.ambient || s.trail || s.click_flare) { return; }
    let Some(player) = game.native_player() else { return; };
    if ![player.x, player.y, player.z].iter().all(|v| v.is_finite()) { return; }
    let color = m::THEMES[s.theme].2;
    let mut add_light = |at: [f64; 3], radius: f32, intensity: f32| {
        if lights.len() >= 8 || !at.iter().all(|v| v.is_finite()) { return; }
        lights.push(crate::game::native_hooks::NativeColoredLight {
            at, rgb: color, radius, intensity: intensity.clamp(0.0, 1.0),
        });
    };
    if s.ambient {
        add_light([player.x, player.y + 1.0, player.z], 5.0, m::ambient_intensity(s.reduced_motion));
    }
    if s.trail && !s.reduced_motion {
        for p in m::trail_lights().into_iter().take(m::trail_limit(s.adaptive_quality)) {
            let fade = 1.0 - p.age as f32 / 18.0;
            add_light(p.pos, 2.7, (fade * 0.32).max(0.04));
        }
    }
    if s.click_flare {
        let pulse = m::input_pulse();
        let level = pulse.left.max(pulse.right);
        if level > 0 {
            add_light([player.x, player.y + 1.4, player.z], 4.0, level as f32 / 8.0 * 0.50);
        }
    }
}
