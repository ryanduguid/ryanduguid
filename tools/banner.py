"""ASCII ledger banners for the profile and repository READMEs.

Renders the DR and CR ledger banner system: a masthead for the profile README,
a header for each public repository, a startup banner for the command line
tools, and a scope notice. Output is 7 bit ASCII only, so it survives any
codepage, terminal and pager. Box drawing is deliberately not used: it is East
Asian Ambiguous width and renders double width under a CJK configured terminal.

Exit 0 clean, 1 on any check failure. Stdlib only.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

MIN_WIDTH = 24

MAX_WIDTH = 80

CONTENT = ROOT / "tools" / "banner_content.json"

TARGETS = (
    "DiogenesLamp",
    "DrDebits",
    "FireFalcon",
    "Ozzit",
    "PaciolisCube",
    "SolomonsSword",
    "TheExchequerTally",
    "accounting-excel-toolkit",
    "ato-benchmark-compare",
    "au-tax-legislation-corpus",
    "au-tax-mcp-server",
    "australian-accounting-skills",
    "awesome-australian-accounting-tech",
    "hardhat-ledger",
    "monthly-close-control-plane",
    "payday-super-checker",
    "release-policy",
    "xero-ai-review-gateway",
    "xero-trial-balance-export",
)

# Two targets were skipped on Ryan's ruling, 25 August 2026. Do not re-attempt:
# xero-trial-balance-export pins its README SHA-256 as a constant inside a test,
# and release-policy pins its README digest in a hardcoded canonical table.
# Pasting a banner into either means breaking a provenance guard.


def load_content() -> dict[str, dict]:
    """Per repository banner content. Claims here must match that repo's README."""
    return json.loads(CONTENT.read_text(encoding="utf-8"))


class Ledger:
    """A banded banner on a fixed column grid.

    Column 1 and column `width` carry the outer border. A two column band puts
    its divider at `mid`, giving cells of `left` and `right` inner columns.
    """

    def __init__(self, width: int) -> None:
        if width < MIN_WIDTH:
            raise ValueError(f"width {width} is below the minimum of {MIN_WIDTH}")
        if width > MAX_WIDTH:
            raise ValueError(f"width {width} is above the maximum of {MAX_WIDTH}")
        self.width = width
        self.mid = 1 + ((width - 3) // 2) + 1
        self.left = self.mid - 2
        self.right = width - self.mid - 1
        self._lines: list[str] = []
        self._band: int | None = None

    def _rows(self) -> list[str]:
        return self._lines

    def _cell(self, text: str, inner: int, center: bool, blank: str = "-") -> str:
        """One cell body of exactly `inner` columns.

        `blank` is what an empty value renders as: a hyphen in a DR or CR cell,
        nothing in a full width row, because those blank rows are the masthead's
        spacer rows. Truncation with an ellipsis applies to every path. A left
        aligned cell spends one column on its leading space, a centred one does
        not, so a value that fits the cell exactly is never cut short.
        """
        text = text if text else blank
        room = inner if center else inner - 1
        if len(text) > room:
            text = text[: max(0, room - 3)] + "..."
        return text.center(inner) if center else " " + text.ljust(room)

    def _rule_line(self, cols: int) -> str:
        if cols == 1:
            return "+" + "-" * (self.width - 2) + "+"
        return "+" + "-" * self.left + "+" + "-" * self.right + "+"

    def _open(self, cols: int) -> None:
        rows = self._rows()
        if self._band is None:
            rows.append(self._rule_line(cols))
        elif self._band != cols:
            rows.append(self._rule_line(2))
        self._band = cols

    def full(self, text: str = "", center: bool = True) -> "Ledger":
        self._open(1)
        inner = self.width - 2
        body = self._cell(text, inner, center, blank="")
        self._rows().append("|" + body[:inner] + "|")
        return self

    def split(self, left: str, right: str, center: bool = False) -> "Ledger":
        self._open(2)
        a = self._cell(left, self.left, center)
        b = self._cell(right, self.right, center)
        self._rows().append("|" + a[: self.left] + "|" + b[: self.right] + "|")
        return self

    def rule(self) -> "Ledger":
        self._rows().append(self._rule_line(self._band or 1))
        return self

    def render(self) -> str:
        rows = list(self._rows())
        rows.append(self._rule_line(self._band or 1))
        return "\n".join(rows)


def check(name: str, text: str) -> list[str]:
    """Gate one rendered block. Returns failure strings, empty when clean."""
    lines = text.split("\n")
    failures: list[str] = []

    widths = {len(line) for line in lines}
    if len(widths) != 1:
        failures.append(f"{name}: ragged widths {sorted(widths)}")

    if any(line != line.rstrip() for line in lines):
        failures.append(f"{name}: trailing whitespace")

    if any(ord(ch) > 127 for ch in text):
        failures.append(f"{name}: non ASCII character present, output must be 7 bit ASCII")

    for row, line in enumerate(lines, start=1):
        for col, ch in enumerate(line, start=1):
            if ch != "|":
                continue
            # Report once, even when a rule sits both above and below.
            above_or_below = (row - 2, row)
            if any(
                0 <= n < len(lines) and col <= len(lines[n]) and lines[n][col - 1] == "-"
                for n in above_or_below
            ):
                failures.append(
                    f"{name}: junction, pipe at row {row} column {col} "
                    f"meets a rule, needs a plus"
                )
    return failures


def masthead() -> str:
    """The mobile-safe profile README masthead, 34 columns."""
    led = Ledger(34)
    led.full("RYAN DUGUID", center=False)
    led.full("COMPUTATIONAL ACCOUNTING", center=False)
    led.split("DR", "CR")
    led.rule()
    led.split("Excel LAMBDAs", "Newcastle, NSW")
    led.split("MCP + CLI", "CA ANZ (prov.)")
    led.split("Tax + payroll", "SAP / Xero")
    led.full("IN BALANCE")
    return led.render()


def masthead_compact() -> str:
    """The masthead below 60 columns, where lettering is dropped not wrapped."""
    led = Ledger(56)
    led.full("RYAN DUGUID")
    led.rule()
    led.full("computational accounting, Australian tax")
    led.split("DR  Excel, MCP, CLI", "CR  Newcastle NSW")
    return led.render()


def repo_header(name: str, tagline: str, gives: list[str], needs: list[str]) -> str:
    """A repository README header, 72 columns.

    Repository names are set as text, never as glyphs. The name
    payday-super-checker in a 5 by 5 font is 120 columns wide.

    Each column carries between one and three entries. An empty list would
    close the band on the rule that opened it, two rules with no data between.
    """
    if not gives or not needs:
        raise ValueError("gives and needs each need at least one entry")
    led = Ledger(72)
    led.full(name)
    led.rule()
    led.full(tagline)
    led.split("DR  what it gives you", "CR  what it needs")
    led.rule()
    rows = max(len(gives), len(needs))
    for index in range(rows):
        left = gives[index] if index < len(gives) else ""
        right = needs[index] if index < len(needs) else ""
        led.split(left, right)
    return led.render()


def cli_banner(name: str, release: str, command: str) -> str:
    """A command line startup banner, 64 columns.

    The release is written as release 1.4.0. A bare v before a digit reads as a
    down arrow with nothing to connect to, so the constructor rejects one
    rather than trusting every caller to strip it.
    """
    if release[:1] in ("v", "V") and release[1:2].isdigit():
        raise ValueError(
            f"release {release!r} starts with a bare v before a digit, "
            f"write it as {release[1:]!r}"
        )
    led = Ledger(64)
    led.full(name)
    led.rule()
    led.full(f"release {release}   github.com/ryanduguid")
    led.split(f"DR  {command}", "CR  reads local files only")
    return led.render()


def notice_scope() -> str:
    """The scope fence every tax adjacent repository already needs."""
    led = Ledger(72)
    led.full("NOT ADVICE", center=False)
    led.rule()
    for line in (
        "General information about Australian tax rules.",
        "Not tax, legal or financial advice. Verify against the",
        "primary source before relying on any output.",
    ):
        led.full(line, center=False)
    return led.render()


def all_blocks() -> list[tuple[str, str]]:
    """Every block the gate covers, as (name, text)."""
    content = load_content()
    blocks = [
        ("MASTHEAD_FULL", masthead()),
        ("MASTHEAD_COMPACT", masthead_compact()),
        ("CLI_BANNER", cli_banner("payday-super-checker", "1.4.0", "check payroll.csv")),
        ("NOTICE_SCOPE", notice_scope()),
    ]
    for name in TARGETS:
        record = content[name]
        blocks.append(
            (name, repo_header(name, record["tagline"], record["gives"], record["needs"]))
        )
    return blocks


def main(argv: list[str] | None = None) -> int:
    args = list(sys.argv[1:] if argv is None else argv)
    if not args or args[0] == "--check":
        blocks = all_blocks()
        failures: list[str] = []
        for name, text in blocks:
            failures.extend(check(name, text))
        for failure in failures:
            print(failure)
        print(f"{len(blocks)} blocks checked, {len(failures)} failures")
        return 1 if failures else 0

    if args[0] == "masthead":
        print(masthead())
        return 0
    if args[0] == "compact":
        print(masthead_compact())
        return 0
    if args[0] == "notice":
        print(notice_scope())
        return 0
    if args[0] == "repo" and len(args) > 1:
        content = load_content()
        if args[1] not in content:
            print(f"unknown repository: {args[1]}", file=sys.stderr)
            return 1
        record = content[args[1]]
        print(repo_header(args[1], record["tagline"], record["gives"], record["needs"]))
        return 0

    print(
        "usage: banner.py [--check | masthead | compact | notice | repo <name>]",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
