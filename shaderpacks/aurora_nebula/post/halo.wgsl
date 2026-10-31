// Lightweight eight-tap single-pass bloom approximation. This is a soft
// screen-space halo, NOT an HDR pyramidal bloom. Avoids extra targets.
override halo_gain: f32 = 0.28;

fn post(input: PostInput) -> vec4<f32> {
    if halo_gain <= 0.0 {
        return input.color;
    }
    let texel = vec2<f32>(1.0, 1.0) / max(frame.resolution, vec2<f32>(1.0));
    var amount = vec3<f32>(0.0);
    for (var i = 0; i < 8; i = i + 1) {
        let theta = f32(i) * 0.78539816;
        let offset = vec2<f32>(cos(theta), sin(theta)) * texel * 5.0;
        let c = textureSampleLevel(previous, linear_sampler,
                                   clamp(input.uv + offset, vec2<f32>(0.0), vec2<f32>(1.0)), 0.0).rgb;
        let bright = smoothstep(0.68, 0.95, dot(c, vec3<f32>(0.2126, 0.7152, 0.0722)));
        amount = amount + c * bright;
    }
    let rgb = clamp(input.color.rgb + amount * (halo_gain * 0.11),
                    vec3<f32>(0.0), vec3<f32>(1.0));
    return vec4<f32>(rgb, input.color.a);
}
