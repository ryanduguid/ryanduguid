"""ASCII ledger banners for the profile and repository READMEs.

Renders the DR and CR ledger banner system: a masthead for the profile README,
a header for each public repository, a startup banner for the command line
tools, and a scope notice. Output is 7 bit ASCII only, so it survives any
codepage, terminal and pager. Box drawing is deliberately not used: it is East
Asian Ambiguous width and renders double width under a CJK configured terminal.

Exit 0 clean, 1 on any check failure. Stdlib only.
"""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

MIN_WIDTH = 24

WORDMARK = Path(__file__).resolve().parent / "wordmark.txt"


class Ledger:
    """A banded banner on a fixed column grid.

    Column 1 and column `width` carry the outer border. A two column band puts
    its divider at `mid`, giving cells of `left` and `right` inner columns.
    """

    def __init__(self, width: int) -> None:
        if width < MIN_WIDTH:
            raise ValueError(f"width {width} is below the minimum of {MIN_WIDTH}")
        self.width = width
        self.mid = 1 + ((width - 3) // 2) + 1
        self.left = self.mid - 2
        self.right = width - self.mid - 1
        self._lines: list[str] = []
        self._band: int | None = None

    def _rows(self) -> list[str]:
        return self._lines

    def _cell(self, text: str, inner: int, center: bool) -> str:
        text = text if text else "-"
        room = inner - 1
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
        # Empty values stay blank in full() rows (masthead spacers).
        # Hyphenation is for split() cells only.
        room = inner if center else inner - 1
        if len(text) > room:
            text = text[: max(0, room - 3)] + "..."
        body = text.center(inner) if center else " " + text.ljust(inner - 1)
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

    def art(self, lines: list[str]) -> "Ledger":
        block = max(len(line) for line in lines)
        pad = (self.width - 2 - block) // 2
        for line in lines:
            self._open(1)
            body = " " * pad + line
            self._rows().append("|" + body.ljust(self.width - 2)[: self.width - 2] + "|")
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
            if ch == "|":
                for step in (-1, 1):
                    neighbour = row - 1 + step
                    if 0 <= neighbour < len(lines):
                        other = lines[neighbour]
                        if col <= len(other) and other[col - 1] == "-":
                            failures.append(
                                f"{name}: junction, pipe at row {row} column {col} "
                                f"meets a rule, needs a plus"
                            )
    return failures


def load_wordmark() -> list[str]:
    """The vendored FIGlet Rectangles rendering of RYAN DUGUID, 64 by 4.

    Regenerate with: npx figlet-cli -f Rectangles "RYAN DUGUID"
    """
    rows = WORDMARK.read_text(encoding="ascii").split("\n")
    rows = [row for row in rows if row.strip()]
    block = max(len(row) for row in rows)
    return [row.ljust(block) for row in rows]


def masthead() -> str:
    """The profile README masthead, 72 columns."""
    led = Ledger(72)
    led.full("")
    led.art(load_wordmark())
    led.full("")
    led.split("DR", "CR", center=True)
    led.rule()
    led.split("Excel LAMBDA engines", "Newcastle, NSW")
    led.split("MCP servers, CLI tools", "CA ANZ (prov), SAP, Xero")
    led.split("Australian tax and payroll", "github.com/ryanduguid")
    led.full("in balance")
    return led.render()


def masthead_compact() -> str:
    """The masthead below 60 columns, where lettering is dropped not wrapped."""
    led = Ledger(56)
    led.full("RYAN DUGUID")
    led.rule()
    led.full("computational accounting, Australian tax")
    led.split("DR  Excel, MCP, CLI", "CR  Newcastle NSW")
    return led.render()
