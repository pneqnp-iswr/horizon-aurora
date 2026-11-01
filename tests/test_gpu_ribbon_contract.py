"""Offline invariants before invoking official Horizon native compiler.

This does not claim GPU correctness or measure performance.
"""
import json
import re
import tomllib
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
MOD = ROOT / "mixins" / "aurora_visuals"


class NativeGpuRibbonContract(unittest.TestCase):
    def test_api58_and_cross_crate_module(self):
        manifest = tomllib.loads((MOD / "horizon-mixin.toml").read_text())
        compatibility = json.loads((ROOT / "compatibility.json").read_text())
        self.assertGreaterEqual(compatibility["hooks"], 58)
        self.assertEqual(len(manifest["module"]), 2)
        gpu = next(x for x in manifest["module"] if x["id"] == "gpu_renderer")
        self.assertEqual(gpu["visibility"], "pub")
        self.assertEqual(gpu["file"], "crates/horizon-client-render/src/gfx/native_render.rs")
        self.assertEqual(gpu["source"], "ribbon_gpu.rs")
        injections = {x["id"]: x for x in manifest["injection"]}
        self.assertIn("native_render_needs", injections["gpu_needs"]["target"])
        self.assertIn("native_world_drawn", injections["gpu_world"]["target"])
        self.assertEqual(manifest["compatibility"]["horizon_revision"], compatibility["horizon_revision"])

    def test_real_vertex_pipeline_and_depth_semantics(self):
        gpu = (MOD / "ribbon_gpu.rs").read_text()
        for marker in (
            "create_render_pipeline(", "pass.draw(", "view_proj.to_cols_array()",
            "WorldDrawn<'_>", "world_depth = true", "reversed depth",
            "textureLoad(scene_depth", "TextureSampleType::Depth",
            "ALPHA_BLENDING", "POINT_LIMIT: usize = 72",
            "VERTEX_LIMIT: usize = 8192", "relative(p: [f64;3]",
            "device_address", "world.queue.write_buffer",
            "spline(", "pub fn publish(", "pub fn inactive(",
        ):
            with self.subTest(marker=marker):
                self.assertIn(marker, gpu)
        self.assertNotIn("native_flame_particle", gpu)
        self.assertNotRegex(gpu, r"(?m)^\s*#!")
        self.assertNotRegex(gpu, r"(?m)^\s*//!")

    def test_game_snapshot_and_live_controls(self):
        tick = (MOD / "tick.rs").read_text()
        cmd = (MOD / "command.rs").read_text()
        core = (MOD / "runtime.rs").read_text()
        self.assertIn("aurora_gpu::publish(", tick)
        self.assertIn("aurora_gpu::inactive()", tick)
        self.assertIn('ribbon-style silk|prismatic|spectral', cmd)
        self.assertIn('ribbon-demo', cmd)
        self.assertIn('ribbon_enabled: false', core)
        self.assertIn('"ribbon_enabled"', core)
        self.assertIn('"ribbon_style"', core)
        self.assertIn('"ribbon_demo"', core)
        self.assertIn("fn settings_round_trip()", core)


if __name__ == "__main__":
    unittest.main()
