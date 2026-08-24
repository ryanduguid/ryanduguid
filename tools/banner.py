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
        body = text.center(inner) if center else self._cell(text, inner, False)
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
        return "\n".join(row.rstrip("\n") for row in rows)
