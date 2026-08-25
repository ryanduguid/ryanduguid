"""Checks for tools/banner.py. Run: python -m unittest discover -s tools -p "test_*.py" """

from __future__ import annotations

import contextlib
import io
import unittest

from banner import (
    CONTENT,
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


class TestProfileOpening(unittest.TestCase):
    def setUp(self):
        self.readme = (ROOT / "README.md").read_text(encoding="utf-8")
        self.opening = self.readme.split("\n## ", maxsplit=1)[0]

    def test_the_opening_identifies_the_person_and_the_two_install_routes(self):
        for anchor in (
            "Australian accountant",
            "Newcastle, NSW",
            "computational accounting",
            "aus-accounting-mcp",
            "australian-accounting-skills",
        ):
            with self.subTest(anchor=anchor):
                self.assertIn(anchor, self.opening)

    def test_the_first_section_heading_precedes_the_generated_masthead(self):
        self.assertLess(self.readme.index("\n## "), self.readme.index(masthead()))

    def test_the_private_off_ledger_markers_are_absent(self):
        self.assertNotIn("off-ledger:", self.readme)
        self.assertNotIn("callsign:", self.readme)

    def test_the_profile_links_to_the_three_canonical_proof_pages(self):
        for url in (
            "https://ryanduguid.github.io/evidence/",
            "https://ryanduguid.github.io/tools/payday-super/",
            "https://ryanduguid.github.io/tools/xero-trial-balance/",
        ):
            with self.subTest(url=url):
                self.assertIn(url, self.readme)

    def test_the_profile_has_the_approved_tooling_attribution_footer(self):
        attribution = (
            "Orchestrated with [Hermes Agent]"
            "(https://github.com/NousResearch/hermes-agent)."
        )
        self.assertEqual(1, self.readme.count(attribution))
        self.assertTrue(self.readme.rstrip().endswith(attribution))
        self.assertNotRegex(
            self.readme,
            r"(?mi)^Built (?:by|using)\b.*Hermes Agent",
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

    def test_rejects_a_width_above_the_maximum(self):
        Ledger(80)
        with self.assertRaises(ValueError):
            Ledger(81)


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

    def test_a_centred_value_that_fills_the_cell_exactly_is_not_cut_short(self):
        led = Ledger(72)
        out = led.split("x" * led.left, "y" * led.right, center=True).render()
        self.assertNotIn("...", out)
        self.assertIn("x" * led.left, out)
        self.assertIn("y" * led.right, out)

    def test_art_wider_than_the_frame_is_rejected_not_clipped(self):
        with self.assertRaises(ValueError):
            Ledger(72).art(["A" * 80, "B" * 80])


class TestGate(unittest.TestCase):
    def test_a_clean_banner_passes(self):
        text = Ledger(72).full("title").split("a", "b").render()
        self.assertEqual(check("clean", text), [])

    def test_a_ragged_block_fails(self):
        # Two rules, no pipes, no trailing space, so only the width rule fires.
        failures = check("ragged", "+--+\n+-+")
        self.assertEqual(len(failures), 1, failures)
        self.assertIn("ragged", failures[0])

    def test_a_non_ascii_character_fails(self):
        text = Ledger(72).full("café").render()
        failures = check("unicode", text)
        self.assertTrue(any("ASCII" in f for f in failures))

    def test_trailing_whitespace_fails(self):
        # Equal widths and no pipes, so only the trailing whitespace rule fires.
        failures = check("trailing", "+--+ \n+--+ ")
        self.assertEqual(len(failures), 1, failures)
        self.assertIn("trailing", failures[0])

    def test_a_stem_sitting_on_a_rule_fails(self):
        text = "+----+\n| || |\n+----+"
        self.assertTrue(any("junction" in f for f in check("stem", text)))

    def test_a_pipe_ruled_above_and_below_is_reported_once(self):
        failures = check("both", "-\n|\n-")
        self.assertEqual(len(failures), 1, failures)
        self.assertIn("junction", failures[0])


class TestMasthead(unittest.TestCase):
    def test_the_wordmark_is_a_rectangle_of_the_expected_size(self):
        rows = load_wordmark()
        self.assertEqual(len(rows), 5)
        self.assertEqual({len(r) for r in rows}, {60})

    def test_the_masthead_passes_the_gate(self):
        self.assertEqual(check("masthead", masthead()), [])

    def test_the_spacer_rows_are_load_bearing(self):
        lines = masthead().split("\n")
        last_spacer = 1 + len(load_wordmark()) + 1
        stripped = [
            line
            for index, line in enumerate(lines)
            if index not in (1, last_spacer)
        ]
        self.assertTrue(check("no spacer", "\n".join(stripped)))

    def test_the_wordmark_band_is_padded_so_no_stem_touches_a_rule(self):
        lines = masthead().split("\n")
        last_spacer = 1 + len(load_wordmark()) + 1
        self.assertEqual(lines[1].strip("|").strip(), "")
        self.assertEqual(lines[last_spacer].strip("|").strip(), "")

    def test_the_masthead_closes_on_the_balance_line(self):
        self.assertIn("in balance", masthead().split("\n")[-2])

    def test_the_readme_carries_the_masthead_the_generator_renders(self):
        # The pasted artefact is what ships, so gate it against the generator.
        readme = (ROOT / "README.md").read_text(encoding="utf-8")
        self.assertIn(masthead(), readme)

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
        name = max(TARGETS, key=len)
        out = repo_header(name, "tagline", ["a"], ["b"])
        self.assertEqual(check("longest", out), [])
        self.assertIn(name, out.split("\n")[1])

    def test_a_repo_header_needs_at_least_one_give_and_one_need(self):
        with self.assertRaises(ValueError):
            repo_header("x", "y", [], ["d"])
        with self.assertRaises(ValueError):
            repo_header("x", "y", ["a"], [])

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

    def test_the_cli_banner_rejects_a_bare_v_version(self):
        for bad in ("v1.4.0", "V1.4.0", "v2"):
            with self.assertRaises(ValueError, msg=bad):
                cli_banner("tool", bad, "run")

    def test_the_cli_banner_writes_a_clean_version_as_release(self):
        out = cli_banner("tool", "1.4.0", "run")
        self.assertIn("release 1.4.0   github.com/ryanduguid", out)
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
            gives, needs = record["gives"], record["needs"]
            out = repo_header(name, record["tagline"], gives, needs)
            self.assertEqual(check(name, out), [], f"{name} failed the gate")
            lines = out.split("\n")
            self.assertIn(name, lines[1], name)
            self.assertIn(record["tagline"], lines[3], name)
            # Data rows start at index 7, DR on the left of the divider, CR on
            # the right. Nothing is truncated, so every value arrives whole.
            for index in range(max(len(gives), len(needs))):
                _, dr, cr, _ = lines[7 + index].split("|")
                if index < len(gives):
                    self.assertIn(gives[index], dr, f"{name} gives row {index}")
                if index < len(needs):
                    self.assertIn(needs[index], cr, f"{name} needs row {index}")

    def test_every_record_carries_between_one_and_three_of_each_column(self):
        content = load_content()
        for name in TARGETS:
            record = content[name]
            self.assertTrue(1 <= len(record["gives"]) <= 3, name)
            self.assertTrue(1 <= len(record["needs"]) <= 3, name)

    def test_no_record_uses_an_em_dash_or_en_dash(self):
        raw = CONTENT.read_text(encoding="utf-8")
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
