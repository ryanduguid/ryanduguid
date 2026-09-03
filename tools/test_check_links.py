"""Policy tests for profile URL extraction and automation denials."""

from __future__ import annotations

import contextlib
import io
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import check_links


class UrlPolicyTests(unittest.TestCase):
    def test_parenthesised_machine_index_links_cannot_hide_rename_redirects(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "llms.txt").write_text(
                "- **MCP** (https://github.com/ryanduguid/aus-accounting-mcp): local tool\n",
                encoding="utf-8",
            )
            output = io.StringIO()
            with (
                patch.object(check_links, "ROOT", root),
                patch.object(check_links, "FILES", ["llms.txt"]),
                patch.object(check_links, "fetch_final_url", return_value=(
                    200, "https://github.com/ryanduguid/australian-accounting"
                )),
                contextlib.redirect_stdout(output),
            ):
                self.assertEqual(check_links.main(), 1)
            self.assertIn("rename redirect, repoint the link", output.getvalue())

    def test_normalises_markdown_code_span_url(self) -> None:
        self.assertEqual(
            check_links.normalise_url("https://duguid.com.au/`"),
            "https://duguid.com.au/",
        )

    def test_accepts_only_the_hibernated_linkedin_automation_denial(self) -> None:
        linkedin = "https://www.linkedin.com/in/ryan-duguid/"

        self.assertTrue(check_links.is_accepted_automation_denial(linkedin, 999))
        self.assertFalse(check_links.is_accepted_automation_denial(linkedin, 404))
        self.assertFalse(
            check_links.is_accepted_automation_denial(
                "https://www.linkedin.com/in/someone-else/", 999
            )
        )


if __name__ == "__main__":
    unittest.main()
