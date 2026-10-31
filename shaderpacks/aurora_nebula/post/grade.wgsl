// Gentle material-preserving cinematic finishing pass; underlying gameplay
// is not changed. Frame's day/night and weather drive hue adaptation.
override grade_strength: f32 = 0.42;
override vignette_strength: f32 = 0.28;

fn post(input: PostInput) -> vec4<f32> {
    var rgb = input.color.rgb;
    let night = 1.0 - smoothstep(-0.14, 0.06, frame.sun_direction.y);
    let tint = vec3<f32>(0.94, 0.98, 1.08);
    rgb = mix(rgb, rgb * tint, grade_strength * night * 0.65);
    let luma = dot(rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    rgb = mix(rgb, vec3<f32>(luma), grade_strength * 0.13);
    let coord = input.uv - vec2<f32>(0.5, 0.5);
    let ratio = max(frame.resolution.x, 1.0) / max(frame.resolution.y, 1.0);
    let radial = length(coord * vec2<f32>(ratio, 1.0)) /
                 length(vec2<f32>(0.5 * ratio, 0.5));
    let darken = smoothstep(0.46, 1.03, radial) * vignette_strength;
    rgb = rgb * (1.0 - darken);
    return vec4<f32>(clamp(rgb, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
