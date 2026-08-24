"""Checks for tools/banner.py. Run: python -m unittest discover -s tools -p "test_*.py" """

from __future__ import annotations

import contextlib
import io
import unittest

from banner import (
    ROOT,
    TARGETS,
    Ledger,
    check,
    cli_banner,
    load_content,
    load_wordmark,
    main,
    masthead,
    masthead_compact,
    notice_scope,
    repo_header,
)


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


class TestMasthead(unittest.TestCase):
    def test_the_wordmark_is_a_rectangle_of_the_expected_size(self):
        rows = load_wordmark()
        self.assertEqual(len(rows), 4)
        self.assertEqual({len(r) for r in rows}, {64})

    def test_the_masthead_passes_the_gate(self):
        self.assertEqual(check("masthead", masthead()), [])

    def test_the_spacer_rows_are_load_bearing(self):
        lines = masthead().split("\n")
        stripped = [line for index, line in enumerate(lines) if index not in (1, 6)]
        self.assertTrue(check("no spacer", "\n".join(stripped)))

    def test_the_wordmark_band_is_padded_so_no_stem_touches_a_rule(self):
        lines = masthead().split("\n")
        self.assertEqual(lines[1].strip("|").strip(), "")
        self.assertEqual(lines[6].strip("|").strip(), "")

    def test_the_masthead_closes_on_the_balance_line(self):
        self.assertIn("in balance", masthead().split("\n")[-2])

    def test_the_compact_masthead_drops_the_lettering_below_sixty(self):
        out = masthead_compact()
        self.assertEqual({len(line) for line in out.split("\n")}, {56})
        self.assertNotIn("_____", out)
        self.assertEqual(check("compact", out), [])


class TestTemplates(unittest.TestCase):
    def test_a_repo_header_passes_the_gate(self):
        out = repo_header(
            "payday-super-checker",
            "SG charge and due dates since 1 July 2026",
            ["due date per payday event"],
            ["payroll export CSV"],
        )
        self.assertEqual(check("repo", out), [])
        self.assertEqual({len(line) for line in out.split("\n")}, {72})

    def test_the_longest_repository_name_still_fits(self):
        out = repo_header("awesome-australian-accounting-tech", "tagline", ["a"], ["b"])
        self.assertEqual(check("longest", out), [])

    def test_uneven_gives_and_needs_do_not_drop_rows(self):
        out = repo_header("x", "y", ["a", "b", "c"], ["d"]).split("\n")
        data = [line for line in out if line[1:3] in (" a", " b", " c")]
        self.assertEqual(len(data), 3)
        self.assertTrue(data[1].endswith("| -" + " " * 33 + "|"))
        self.assertEqual(check("uneven", "\n".join(out)), [])

    def test_the_cli_banner_is_sixty_four_columns_and_passes(self):
        out = cli_banner("payday-super-checker", "1.4.0", "check payroll.csv")
        self.assertEqual({len(line) for line in out.split("\n")}, {64})
        self.assertEqual(check("cli", out), [])

    def test_the_cli_banner_never_writes_a_bare_v_version(self):
        out = cli_banner("tool", "1.4.0", "run")
        self.assertIn("release 1.4.0", out)
        self.assertNotIn("v1.4.0", out)

    def test_the_scope_notice_passes(self):
        self.assertEqual(check("notice", notice_scope()), [])


class TestContent(unittest.TestCase):
    def test_every_target_has_a_record(self):
        content = load_content()
        for name in TARGETS:
            self.assertIn(name, content)

    def test_there_are_exactly_twenty_targets(self):
        self.assertEqual(len(TARGETS), 20)
        self.assertEqual(len(set(TARGETS)), 20)

    def test_no_private_or_excluded_repository_is_targeted(self):
        excluded = {
            "ryanduguid", ".github", "ryanduguid.github.io",
            "ChestertonsFence", "Furphy", "claude-export",
        }
        self.assertEqual(excluded & set(TARGETS), set())

    def test_every_record_renders_a_banner_that_passes_the_gate(self):
        content = load_content()
        for name in TARGETS:
            record = content[name]
            out = repo_header(name, record["tagline"], record["gives"], record["needs"])
            self.assertEqual(check(name, out), [], f"{name} failed the gate")

    def test_every_record_carries_between_one_and_three_of_each_column(self):
        content = load_content()
        for name in TARGETS:
            record = content[name]
            self.assertTrue(1 <= len(record["gives"]) <= 3, name)
            self.assertTrue(1 <= len(record["needs"]) <= 3, name)

    def test_no_record_uses_an_em_dash_or_en_dash(self):
        raw = (ROOT / "tools" / "banner_content.json").read_text(encoding="utf-8")
        self.assertNotIn("\u2014", raw)
        self.assertNotIn("\u2013", raw)


class TestCommandLine(unittest.TestCase):
    def test_check_mode_passes_on_the_shipped_content(self):
        buffer = io.StringIO()
        with contextlib.redirect_stdout(buffer):
            code = main(["--check"])
        self.assertEqual(code, 0, buffer.getvalue())

    def test_masthead_mode_prints_the_masthead(self):
        buffer = io.StringIO()
        with contextlib.redirect_stdout(buffer):
            code = main(["masthead"])
        self.assertEqual(code, 0)
        self.assertIn("in balance", buffer.getvalue())

    def test_repo_mode_prints_that_repository_header(self):
        buffer = io.StringIO()
        with contextlib.redirect_stdout(buffer):
            code = main(["repo", "payday-super-checker"])
        self.assertEqual(code, 0)
        self.assertIn("payday-super-checker", buffer.getvalue())

    def test_an_unknown_repository_is_an_error_not_a_traceback(self):
        buffer = io.StringIO()
        with contextlib.redirect_stderr(buffer):
            code = main(["repo", "no-such-repo"])
        self.assertEqual(code, 1)
        self.assertIn("no-such-repo", buffer.getvalue())


if __name__ == "__main__":
    unittest.main()
