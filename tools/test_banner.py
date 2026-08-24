"""Checks for tools/banner.py. Run: python -m unittest discover -s tools -p "test_*.py" """

from __future__ import annotations

import unittest

from banner import Ledger, check


class TestGeometry(unittest.TestCase):
    def test_divider_column_at_72(self):
        led = Ledger(72)
        self.assertEqual(led.mid, 36)
        self.assertEqual(led.left, 34)
        self.assertEqual(led.right, 35)

    def test_divider_column_at_64(self):
        led = Ledger(64)
        self.assertEqual(led.mid, 32)
        self.assertEqual(led.left, 30)
        self.assertEqual(led.right, 31)

    def test_cells_and_borders_fill_the_width(self):
        for width in (56, 64, 72):
            led = Ledger(width)
            self.assertEqual(1 + led.left + 1 + led.right + 1, width)

    def test_rejects_a_width_too_narrow_to_hold_two_cells(self):
        with self.assertRaises(ValueError):
            Ledger(23)


class TestRendering(unittest.TestCase):
    def test_every_line_is_exactly_the_width(self):
        out = Ledger(72).full("title").split("a", "b").render().split("\n")
        self.assertEqual({len(line) for line in out}, {72})

    def test_a_band_change_emits_one_rule_carrying_the_junction(self):
        out = Ledger(72).full("title").split("a", "b").render().split("\n")
        self.assertEqual(out[2], "+" + "-" * 34 + "+" + "-" * 35 + "+")

    def test_split_text_is_left_aligned_with_one_leading_space(self):
        out = Ledger(72).split("gives", "needs").render().split("\n")
        self.assertTrue(out[1].startswith("| gives"))
        self.assertEqual(out[1][35], "|")

    def test_overlong_cell_text_is_truncated_with_an_ellipsis(self):
        out = Ledger(72).split("x" * 100, "y").render().split("\n")
        self.assertEqual(len(out[1]), 72)
        self.assertIn("...", out[1])

    def test_an_empty_value_renders_as_a_hyphen(self):
        out = Ledger(72).split("", "y").render().split("\n")
        self.assertTrue(out[1].startswith("| -"))

    def test_no_line_carries_trailing_whitespace(self):
        out = Ledger(72).full("title").split("a", "b").render().split("\n")
        for line in out:
            self.assertEqual(line, line.rstrip())

    def test_art_block_takes_one_offset_for_every_row(self):
        art = ["####", "#", "####"]
        out = Ledger(72).art(art).render().split("\n")
        starts = [line.index("#") for line in out if "#" in line]
        self.assertEqual(len(set(starts)), 1)

    def test_overlong_centred_text_in_full_is_truncated_with_ellipsis(self):
        out = Ledger(24).full("x" * 100).render().split("\n")
        self.assertEqual(len(out[1]), 24)
        self.assertIn("...", out[1])

    def test_full_with_empty_string_renders_as_blank_row(self):
        out = Ledger(72).full("").render().split("\n")
        self.assertEqual(out[1], "|" + " " * 70 + "|")

    def test_full_left_aligned_with_overlong_text_has_three_dot_ellipsis(self):
        out = Ledger(24).full("x" * 100, center=False).render().split("\n")
        self.assertEqual(len(out[1]), 24)
        self.assertEqual(out[1].count("."), 3)


class TestGate(unittest.TestCase):
    def test_a_clean_banner_passes(self):
        text = Ledger(72).full("title").split("a", "b").render()
        self.assertEqual(check("clean", text), [])

    def test_a_ragged_block_fails(self):
        self.assertTrue(check("ragged", "+---+\n|  |\n+---+"))

    def test_a_non_ascii_character_fails(self):
        text = Ledger(72).full("café").render()
        failures = check("unicode", text)
        self.assertTrue(any("ASCII" in f for f in failures))

    def test_trailing_whitespace_fails(self):
        self.assertTrue(check("trailing", "+--+\n|  | \n+--+"))

    def test_a_stem_sitting_on_a_rule_fails(self):
        text = "+----+\n| || |\n+----+"
        self.assertTrue(any("junction" in f for f in check("stem", text)))


if __name__ == "__main__":
    unittest.main()
