"""Link and content checks for the profile repository.

Scans README.md, llms.txt, SECURITY.md and docs/*.md for Markdown links,
HTML href/src attributes and bare URLs, then checks in order:

1. Every link is https, never http.
2. Every github.com/ryanduguid/<repo> link resolves to that exact repository.
   A rename redirect (301 to a different repo path) is a FAILURE even though
   the request ends in a 200, because redirects break if the old name is reused.
3. Every other absolute link resolves (2xx after redirects).
4. Retired repository names and em or en dashes must not appear outside the
   allowed history notes in docs/MAINTAINING.md.

Exit 0 clean, 1 on any failure. Stdlib only.
"""

from __future__ import annotations

import re
import sys
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILES = ["README.md", "llms.txt", "SECURITY.md", *sorted(
    str(p.relative_to(ROOT)) for p in (ROOT / "docs").glob("*.md")
)]

RETIRED_NAMES = [
    "CharlesHenryWickens",
    "JohnSpenceOgilvy",
    "MaryAddisonHamilton",
    "SirAlexanderFitzgerald",
    "RaymondChambers",
    "SirArthurFadden",
    "RussellMathews",
    "ElizabethAnneAlexander",
    "JohnKenley",
    "EdwinNixon",
    "LouisGoldberg",
]

# MAINTAINING.md legitimately names retired repositories twice: the rename
# history example and the banned-names list itself.
RETIRED_NAME_ALLOWANCE = {"docs/MAINTAINING.md": 2}

USER_AGENT = "ryanduguid-profile-link-check"
LINKEDIN_IDENTITY_URL = "https://www.linkedin.com/in/ryan-duguid/"

LINK_RES = [
    re.compile(r"\[[^\]]*\]\((https?://[^)\s]+)\)"),
    re.compile(r"(?:href|src)=\"(https?://[^\"]+)\""),
    re.compile(r"(?<![(\"=\]])(https?://[^\s)\">\]]+)"),
]


def fetch_final_url(url: str) -> tuple[int, str]:
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=30) as resp:
        return resp.status, resp.geturl()


def normalise_url(url: str) -> str:
    """Remove prose punctuation and Markdown code-span delimiters."""
    return url.rstrip(".,;:`")


def is_accepted_automation_denial(url: str, status: int) -> bool:
    """Accept only LinkedIn's response for the exact identity URL."""
    return url == LINKEDIN_IDENTITY_URL and status == 999


def main() -> int:
    failures: list[str] = []
    urls: dict[str, str] = {}

    for rel in FILES:
        text = (ROOT / rel).read_text(encoding="utf-8")
        for pattern in LINK_RES:
            for url in pattern.findall(text):
                url = normalise_url(url)
                urls.setdefault(url, rel)
        hits = sum(text.count(name) for name in RETIRED_NAMES)
        allowed_lines = RETIRED_NAME_ALLOWANCE.get(rel.replace("\\", "/"), 0)
        if allowed_lines:
            lines_with_hits = sum(
                1 for line in text.splitlines() if any(n in line for n in RETIRED_NAMES)
            )
            if lines_with_hits > allowed_lines:
                failures.append(
                    f"{rel}: retired names on {lines_with_hits} lines, only {allowed_lines} allowed"
                )
        elif hits:
            failures.append(f"{rel}: {hits} retired repository name reference(s)")
        for ch, label in (("—", "em dash"), ("–", "en dash")):
            if ch in text:
                failures.append(f"{rel}: {label} present")

    own_repo = re.compile(r"^https://github\.com/ryanduguid/([A-Za-z0-9._-]+)")
    checked = 0
    for url, src in sorted(urls.items()):
        if url.startswith("http:"):
            failures.append(f"{src}: insecure link {url}")
            continue
        # Badge URLs encode label text, not a resource that can 404 meaningfully.
        if url.startswith("https://img.shields.io/badge/"):
            continue
        checked += 1
        try:
            status, final = fetch_final_url(url)
        except urllib.error.HTTPError as exc:
            if is_accepted_automation_denial(url, exc.code):
                print(
                    f"accepted automation denial {url} -> HTTP {exc.code} "
                    "(exact hibernated LinkedIn identity URL)"
                )
            else:
                failures.append(f"{src}: {url} -> HTTP {exc.code}")
            continue
        except Exception as exc:  # noqa: BLE001 - report every failure mode
            failures.append(f"{src}: {url} -> {exc}")
            continue
        if status >= 400:
            failures.append(f"{src}: {url} -> HTTP {status}")
            continue
        m = own_repo.match(url)
        if m:
            fm = own_repo.match(final)
            if not fm or fm.group(1).lower() != m.group(1).lower():
                failures.append(
                    f"{src}: {url} redirected to {final} (rename redirect, repoint the link)"
                )
        print(f"ok {url}")

    if failures:
        print(f"\n{len(failures)} failure(s):")
        for f in failures:
            print(f"  FAIL {f}")
        return 1
    print(f"\nall clear: {checked} links resolved across {len(FILES)} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
