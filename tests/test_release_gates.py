"""Release invariants: never overwrite a published tag or auto-publish on merge.

Structural validation only. GitHub integration and real gameplay are separate gates.
"""
import re
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class ReleaseGateContract(unittest.TestCase):
    def test_new_candidate_has_consistent_source_versions(self):
        cargo = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
        mod = tomllib.loads((ROOT / "mixins/aurora_visuals/horizon-mixin.toml").read_text(encoding="utf-8"))
        shader = tomllib.loads((ROOT / "shaderpacks/aurora_nebula/horizon-shaders.toml").read_text(encoding="utf-8"))
        runtime = (ROOT / "mixins/aurora_visuals/runtime.rs").read_text(encoding="utf-8")
        version = "0.7.2"
        self.assertEqual(cargo["package"]["version"], version)
        self.assertEqual(mod["mod"]["version"], version)
        self.assertEqual(shader["version"], version)
        self.assertIn(f'pub const VERSION: &str = "{version}";', runtime)

    def test_manual_gameplay_evidence_required_before_release(self):
        workflow = (ROOT / ".github/workflows/verify.yml").read_text(encoding="utf-8")
        self.assertIn("gameplay_evidence_url:", workflow)
        self.assertIn("required: true", workflow)
        self.assertIn("github.event_name == 'workflow_dispatch'", workflow)
        self.assertIn("inputs.gameplay_evidence_url != ''", workflow)
        self.assertNotIn("if: github.event_name == 'push' && github.ref == 'refs/heads/main'", workflow)
        self.assertIn("EVIDENCE_URL:", workflow)
        self.assertIn("TAG: v0.7.2-rc.1", workflow)
        self.assertIn("gh release view", workflow)
        self.assertIn("leaving published assets unchanged", workflow)
        self.assertIn("aurora-v0.7.2-verified-bundles", workflow)
        self.assertIn("aurora_visuals-0.7.2.hnmod", workflow)
        self.assertIn("aurora_nebula-0.7.2.hshader", workflow)
        self.assertNotIn("TAG: v0.7.1-rc.1", workflow)
        self.assertEqual(len(re.findall(r"\bTAG: v0\.7\.2-rc\.1\b", workflow)), 1)


if __name__ == "__main__":
    unittest.main()
