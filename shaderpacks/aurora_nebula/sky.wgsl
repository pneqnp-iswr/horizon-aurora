// Aurora Nebula: procedural, world-direction-driven sky with no textures.
// The prelude supplies SkyInput and `frame`. Vanilla celestial bodies and
// clouds are drawn above this stage, respecting weather and game time.
override aurora_strength: f32 = 0.78;
override nebula_strength: f32 = 0.40;
override starfield_strength: f32 = 0.50;

fn sky(input: SkyInput) -> vec4<f32> {
    let d = input.direction;
    // Sun below the horizon: gradual transition, daylight gameplay unchanged.
    let night = 1.0 - smoothstep(-0.14, 0.05, frame.sun_direction.y);
    let weather = 1.0 - frame.rain * 0.78 - frame.thunder * 0.12;
    let north = smoothstep(-0.10, 0.58, -d.z);
    let above = smoothstep(-0.06, 0.16, d.y) * (1.0 - smoothstep(0.75, 1.0, d.y));
    let flow = frame.time * 0.11;
    let warp = sin(d.x * 11.0 + flow) * 0.06
        + sin(d.x * 27.0 - flow * 0.7) * 0.025;
    let center_a = 0.24 + warp;
    let center_b = 0.41 - warp * 0.6;
    let diff_a = (d.y - center_a) / 0.115;
    let diff_b = (d.y - center_b) / 0.17;
    let curtain_a = exp(-diff_a * diff_a * 1.7);
    let curtain_b = exp(-diff_b * diff_b * 2.0);
    let strands = 0.42 + 0.58 * smoothstep(0.2, 0.92,
        abs(sin(d.x * 58.0 + sin(d.y * 12.0 - flow * 1.5) * 1.4)));
    let intensity = night * max(weather, 0.05) * north * above;
    let emerald = vec3<f32>(0.06, 0.92, 0.58);
    let violet = vec3<f32>(0.60, 0.30, 1.0);
    let shift = 0.5 + 0.5 * sin(d.x * 9.0 + flow * 0.55);
    let curtain = mix(emerald, violet, shift) *
        (curtain_a * strands + curtain_b * 0.53) * intensity * aurora_strength;
    let haze = (1.0 - smoothstep(0.0, 0.58, abs(d.y))) *
        night * north * max(weather, 0.05) * nebula_strength;
    let mist = mix(vec3<f32>(0.05, 0.22, 0.27),
                   vec3<f32>(0.19, 0.07, 0.31), shift) * haze;
    let deepening = 1.0 - night * 0.37;
    var rgb = input.color.rgb * deepening + curtain + mist;
    // Directional star cells, seeded consistently by view direction.
    // No textures or buffers; only runs when enabled and at night.
    if (starfield_strength > 0.0) {
        let cell = floor(vec2<f32>(d.x * 480.0 + d.z * 173.0, d.y * 420.0));
        let seed = fract(sin(dot(cell, vec2<f32>(127.1, 311.7))) * 43758.5453);
        let star = smoothstep(0.995, 0.9996, seed);
        let sky_mask = smoothstep(0.12, 0.35, d.y);
        let sparkle = 0.72 + 0.28 * sin(frame.time * 0.65 + seed * 21.0);
        rgb += vec3<f32>(0.65, 0.82, 1.0) *
            (star * sky_mask * sparkle * night * max(weather, 0.05) * starfield_strength);
    }
    return vec4<f32>(clamp(rgb, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
