"""Policy tests for profile URL extraction and automation denials."""

from __future__ import annotations

import unittest

import check_links


class UrlPolicyTests(unittest.TestCase):
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
