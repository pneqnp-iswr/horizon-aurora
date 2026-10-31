// Aurora Nebula: procedural, world-direction-driven sky with no textures.
// The prelude supplies SkyInput and `frame`. Vanilla celestial bodies and
// clouds are drawn above this stage, respecting weather and game time.
override aurora_strength: f32 = 0.78;
override nebula_strength: f32 = 0.40;

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
    let rgb = clamp(input.color.rgb * deepening + curtain + mist,
                    vec3<f32>(0.0), vec3<f32>(1.0));
    return vec4<f32>(rgb, input.color.a);
}
