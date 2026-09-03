from __future__ import annotations

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOPIC_REPOSITORIES = (
    "australian-accounting",
    "xero-ledger-review-gate",
    "accounting-review-pipeline",
    "workpaper-review-gate",
    "australian-accounting-power-bi",
)
BANNER_REPOSITORIES = (
    "australian-accounting",
    "xero-ledger-review-gate",
    "accounting-review-pipeline",
)
OLD_GITHUB_URLS = (
    "github.com/ryanduguid/au-tax-mcp-server",
    "github.com/ryanduguid/xero-ai-review-gateway",
    "github.com/ryanduguid/monthly-close-control-plane",
    "github.com/ryanduguid/review-ready-gate",
    "github.com/ryanduguid/au-financial-analytics-pbip",
)


class RepositoryIdentityTests(unittest.TestCase):
    def test_active_profile_surfaces_use_canonical_repositories(self) -> None:
        topics = (ROOT / "tools" / "apply-topics.ps1").read_text(encoding="utf-8")
        banners = "\n".join(
            path.read_text(encoding="utf-8")
            for path in (
                ROOT / "tools" / "banner.py",
                ROOT / "tools" / "banner_content.json",
            )
        )
        active = "\n".join(
            path.read_text(encoding="utf-8")
            for path in (
                ROOT / "llms.txt",
                ROOT / "tools" / "apply-topics.ps1",
                ROOT / "tools" / "banner.py",
                ROOT / "tools" / "banner_content.json",
            )
        )
        for repository in TOPIC_REPOSITORIES:
            self.assertIn(f'"ryanduguid/{repository}" =', topics)
        for repository in BANNER_REPOSITORIES:
            self.assertIn(f'"{repository}"', banners)
        for old_url in OLD_GITHUB_URLS:
            self.assertNotIn(old_url, active)

    def test_profile_component_links_point_to_the_maintained_directories(self) -> None:
        text = (ROOT / "llms.txt").read_text(encoding="utf-8")
        links = dict(re.findall(r"^- \*\*([^*]+)\*\* \((https://[^)]+)\):", text, re.MULTILINE))
        components = {
            "Aus Accounting MCP": "australian-accounting/tree/main/apps/aus-accounting-mcp",
            "payday-super-checker": "australian-accounting/tree/main/packages/payday-super-checker",
            "ato-benchmark-compare": "australian-accounting/tree/main/packages/ato-benchmark-compare",
            "TheExchequerTally": "australian-accounting/tree/main/packages/the-exchequer-tally",
            "SolomonsSword": "australian-accounting/tree/main/packages/solomons-sword",
            "xero-trial-balance-export": "accounting-review-pipeline/tree/main/packages/xero-trial-balance-export",
            "accounting-excel-toolkit": "accounting-review-pipeline/tree/main/adapters/accounting-excel-toolkit",
            "Workpaper Review Gate": "accounting-review-pipeline/tree/main/packages/review-ready-gate",
            "Monthly Close Controls": "accounting-review-pipeline/tree/main/packages/monthly-close-control-plane",
            "Xero Ledger Review Gate": "accounting-review-pipeline/tree/main/packages/elizabeth-anne-alexander",
            "Australian Accounting Power BI": "accounting-review-pipeline/tree/main/apps/australian-accounting-power-bi",
            "Hardhat Ledger workflows": "australian-accounting-skills",
        }
        for name, location in components.items():
            with self.subTest(component=name):
                self.assertEqual(links.get(name), f"https://github.com/ryanduguid/{location}")


if __name__ == "__main__":
    unittest.main()
