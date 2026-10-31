"""Ensure the interactive browser lab is in the repository, not only in chat."""
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class StudioPresence(unittest.TestCase):
    def test_studio_is_committed(self):
        html = (ROOT / "studio" / "aurora-studio.html").read_text(encoding="utf-8")
        for marker in ("<!DOCTYPE html>", "<script>", "renderUI();", "pointermove", "localStorage"):
            self.assertIn(marker, html)
        self.assertGreater(len(html), 30000)


if __name__ == "__main__":
    unittest.main()
