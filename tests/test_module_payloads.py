"""Regression guard for Horizon support-module payloads.

The official horizon-mixins validator rejects module-source file attributes,
shebangs and inner documentation comments (E_MODULE). Scan every [[module]]
declaration, not only Aurora's current runtime.rs.
"""
import re
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INNER_DOC = re.compile(r"^\s*//!")
FILE_DIRECTIVE = re.compile(r"^\s*#!")  # includes #![...] and #!/usr/bin/env ...


def forbidden_directives(source: str) -> list[tuple[int, str]]:
    found = []
    for lineno, line in enumerate(source.splitlines(), 1):
        if INNER_DOC.match(line) or FILE_DIRECTIVE.match(line):
            found.append((lineno, line.strip()))
    return found


class ModulePayloadRegression(unittest.TestCase):
    def test_every_declared_support_module(self):
        manifests = sorted(ROOT.glob("mixins/**/horizon-mixin.toml"))
        self.assertTrue(manifests, "No native mod manifests to audit")
        module_count = 0
        for manifest in manifests:
            data = tomllib.loads(manifest.read_text(encoding="utf-8"))
            for module in data.get("module", []):
                module_count += 1
                relative = module["source"]
                source = (manifest.parent / relative).resolve()
                with self.subTest(manifest=str(manifest.relative_to(ROOT)), module=relative):
                    self.assertTrue(source.is_relative_to(manifest.parent.resolve()),
                                    "Module source escapes the bundle")
                    self.assertTrue(source.is_file(), f"Missing module source: {source}")
                    issues = forbidden_directives(source.read_text(encoding="utf-8-sig"))
                    self.assertFalse(issues, f"E_MODULE forbidden file directives in {relative}: {issues}")
        self.assertGreater(module_count, 0, "No [[module]] declarations were checked")

    def test_regression_examples_are_rejected(self):
        for invalid in (
            "//! crate docs\npub const A: u8 = 1;",
            "  //! indented docs\npub const A: u8 = 1;",
            "#![allow(dead_code)]\nfn f() {}",
            "#!/usr/bin/env python3\nprint('no')",
            "pub const A: u8 = 1;\n#![feature(test)]",
        ):
            with self.subTest(invalid=invalid.splitlines()[0]):
                self.assertTrue(forbidden_directives(invalid))

    def test_normal_comments_and_outer_attributes_are_allowed(self):
        valid = "// normal comment\n/// regular item docs\n#[derive(Clone)]\npub struct Example;"
        self.assertEqual(forbidden_directives(valid), [])


if __name__ == "__main__":
    unittest.main()
