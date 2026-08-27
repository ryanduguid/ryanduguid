from __future__ import annotations

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOPIC_REPOSITORIES = (
    "aus-accounting-mcp",
    "xero-ledger-review-gate",
    "monthly-close-controls",
    "workpaper-review-gate",
    "australian-accounting-power-bi",
)
BANNER_REPOSITORIES = (
    "aus-accounting-mcp",
    "xero-ledger-review-gate",
    "monthly-close-controls",
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
            self.assertIn(repository, topics)
        for repository in BANNER_REPOSITORIES:
            self.assertIn(repository, banners)
        for old_url in OLD_GITHUB_URLS:
            self.assertNotIn(old_url, active)

    def test_profile_index_names_all_canonical_repositories(self) -> None:
        text = (ROOT / "llms.txt").read_text(encoding="utf-8")
        for repository in TOPIC_REPOSITORIES:
            self.assertIn(repository, text)


if __name__ == "__main__":
    unittest.main()
