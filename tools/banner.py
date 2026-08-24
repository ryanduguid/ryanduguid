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
