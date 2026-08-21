# Maintaining this profile

This repository is the GitHub profile README for [@ryanduguid](https://github.com/ryanduguid). It is documentation only: there is no application to build, test or deploy.

How to report a security concern is in [SECURITY.md](../SECURITY.md). Account-wide contribution, support and issue-form defaults live in [ryanduguid/.github](https://github.com/ryanduguid/.github). Do not add a `CONTRIBUTING.md` here; it would override that default for this repository only.

## Files

| File | Role |
| --- | --- |
| `README.md` | Public profile at github.com/ryanduguid |
| `assets/banner.svg` | Profile banner. Keep the `aria-label`, `<title>` and README `alt` text in lockstep |
| `SECURITY.md` | Reporting policy for this documentation-only repository |
| `LICENSE` | CC BY 4.0 for the profile prose |
| `docs/MAINTAINING.md` | This runbook |

## Updating the public README

The lead sentence is the practice area. Authorship (original work vs contribution forks) lives in **This account** at the bottom, where [#19](https://github.com/ryanduguid/ryanduguid/pull/19) moved it after [#18](https://github.com/ryanduguid/ryanduguid/pull/18). Do not move that paragraph back to the top.

**Start here** is for the three different ways in: a CLI (`payday-super-checker`), a workbook (`Ozzit`), and Claude Code skills (`australian-accounting-skills`). Copy install commands from those repositories' READMEs in the same pull request as any command change. Do not invent a fourth featured tool without taking one out; three is the point.

**Also** is the rest of the accounting index, one sentence each. Take the sentence from the project's own README or GitHub description, not from memory.

**Selected work merged into other people's projects** names selected examples. Run this query to verify that each named contribution is still merged and correctly attributed:

```bash
gh search prs "author:ryanduguid is:merged -user:ryanduguid"
```

Use that query to confirm every contribution named in the profile is still merged and
correctly attributed. The profile lists selected work rather than a total, so a new merge
does not itself require a profile edit.

## Pinned repositories

GitHub has no public pin API (`pinItem` is not on the public Mutation type). The intended profile pins are listed below. GitHub's signed-out page and GraphQL response
are the source of truth for whether the change is live; do not update this section from an
unsaved Customize your pins dialog.

1. `payday-super-checker`
2. `Ozzit`
3. `xero-trial-balance-export`
4. `australian-accounting-skills`
5. `ato-benchmark-compare`
6. `DrDebits`

Do not pin `.github`, contribution forks, or Resume-Matcher. After changing pins, check https://github.com/ryanduguid for the heading **Pinned** (not **Popular**).

## Claims that must be checked

| Claim | Source of truth |
| --- | --- |
| Nine Claude Code skills, and which workflows they cover | `australian-accounting-skills/.claude/skills/*/SKILL.md` (nine folders) and that repo's README intro |
| `payday-super-check` command, sample path, `--as-at` example | `payday-super-checker/README.md` and `pyproject.toml` `[project.scripts]` |
| Sample file is in the clone, not in a git-URL install | `payday-super-checker/README.md` "Install" |
| Python 3.10+, no runtime dependencies | `payday-super-checker/pyproject.toml` `requires-python` and `dependencies` |
| Allowable longer periods, fund receipt, experimental estimate, no liability determination | `payday-super-checker/README.md` and `paydaysuper/deadlines.py` |
| 130 LAMBDA functions, native Excel only, no add-ins or macros | `Ozzit/README.md` |
| Ozzit needs Microsoft 365 or Excel 2024 | `Ozzit/README.md` "Requirements" |
| Workbook download URL | Latest GitHub release asset, via `/releases/latest/download/ozzit.xlsx`, not a remembered tag. Local `CHANGELOG.md` can be ahead of the published release |
| Plugin slash commands | `australian-accounting-skills/README.md` "Claude Code plugin". They are Claude Code commands, not shell |
| xero-trial-balance-export refuses an unbalanced file | that repo's README "Use" (balance check before write) |
| monthly-close-control-plane writes no journals and locks no periods | that repo's README: reviewer decides; outputs are exception packs |

## Style

- Australian English (`judgement`, `honouring`, `licence` in prose).
- No em dashes. [#7](https://github.com/ryanduguid/ryanduguid/pull/7) and [#8](https://github.com/ryanduguid/ryanduguid/pull/8) existed to take them out. List separators are a hyphen with spaces (` - `).
- Do not claim current public-practice employment. [#2](https://github.com/ryanduguid/ryanduguid/pull/2) dropped that wording. The tools are for that domain; that is not the same sentence.
- Do not add a LinkedIn link while the profile is inactive. [#16](https://github.com/ryanduguid/ryanduguid/pull/16). That includes the GitHub social-account slot (`gh api user/social_accounts`), not only the README.
- Narrow provenance: original work vs forks. [#5](https://github.com/ryanduguid/ryanduguid/pull/5), then [#18](https://github.com/ryanduguid/ryanduguid/pull/18).
- Counts of skills, functions and merged pull requests are part of the prose. A renamed project (Nabla to Ozzit, [#14](https://github.com/ryanduguid/ryanduguid/pull/14)) is a README change in the same breath as the repository rename.

## What stays off the profile

The index is accounting automation for Australian practice. Leave off:

- [DiogenesLamp](https://github.com/ryanduguid/DiogenesLamp), a Yupoo catalogue viewer
- [claude-export](https://github.com/ryanduguid/claude-export), a private settings snapshot
- [ryanduguid/.github](https://github.com/ryanduguid/.github), account-level community health files

Forks used only to send upstream pull requests stay out of Start here and Also. They are already covered by **This account**.

## Common mistakes

- Putting the payday-super sample command next to a `pip install git+...` one-liner. The sample CSV is not on the package.
- Pinning the Ozzit download to `v3.0.0` (or whichever tag you last saw). The published latest on 21 August 2026 is `v3.0.0`; the Ozzit tree can already describe `v3.1.0`. Use `/releases/latest/download/ozzit.xlsx`.
- Fencing the Claude Code `/plugin` lines as `bash`. They are not a shell session.
- Describing payday super as only a seven-business-day deadline. The checker also implements supported allowable longer periods (s 18C(2) items 1 and 4) and fails closed where the facts do not establish them.
- Listing six of the nine skills and calling it nine. The named set in the skills README is BAS, FBT, Division 7A, STP, month-end close, year-end workpapers and 13-week cashflow; `xero-exports` and `workpaper-tie-out` are the other two.
- Forgetting subject-verb agreement in a one-line description. [#17](https://github.com/ryanduguid/ryanduguid/pull/17).
