# Aurora Nebula 0.4.0 — experimental Shader API 0.1 pack

Three independent Horizon shader stages:

- `sky.wgsl` — procedural northern-light curtains and haze, adjusted by day/night and weather.
- `post/halo.wgsl` — a bounded eight-tap highlight halo, **not** HDR pyramid bloom.
- `post/grade.wgsl` — restrained color tint and vignette.

This is **not** an Iris/OptiFine shader pack. It targets the experimental native Horizon Shader API 0.1.

Run the official validator before installation:

```sh
horizon-shader check shaderpacks/aurora_nebula
```

The supplied offline Python structural check does not run Naga, Vulkan/SPIR-V or DirectX/HLSL translation. Treat the pack as an experimental candidate until the official validator and the game confirm it. Do not claim a measured `gpu_ms` from the manifest's budget target.
