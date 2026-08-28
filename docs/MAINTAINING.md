# Maintaining this profile

This repository is the GitHub profile README for [@ryanduguid](https://github.com/ryanduguid). It is documentation only: there is no application to build, test or deploy.

The canonical website host is `https://duguid.com.au/`. Use that host for public website links in `README.md` and `llms.txt`; do not publish the GitHub Pages deployment address as a competing canonical URL.

How to report a security concern is in [SECURITY.md](../SECURITY.md). Account-wide contribution, support and issue-form defaults live in [ryanduguid/.github](https://github.com/ryanduguid/.github). Do not add a `CONTRIBUTING.md` here; it would override that default for this repository only.

## Files

| File | Role |
| --- | --- |
| `README.md` | Public profile at github.com/ryanduguid |
| `llms.txt` | Compact agent-facing index; keep it aligned with README |
| `SECURITY.md` | Reporting policy for this documentation-only repository |
| `LICENSE` | CC BY 4.0 for the profile prose |
| `docs/MAINTAINING.md` | This runbook |
| `tools/apply-topics.ps1` | Applies the topic sets to the public repositories via `gh api` |
| `.github/workflows/link-check.yml` | CI: resolves profile links and fails on rename redirects |

## Updating the public README

The profile is an index, not a second catalogue of every engine. After the identity sentence, name what a reviewer actually gets (Payday Super exceptions, incomplete packs, unbalanced trial balances), then the three paths, then the installable products and the engines those products call. Keep current repository names (`aus-accounting-mcp`, `xero-ledger-review-gate`, `monthly-close-controls`, `workpaper-review-gate`, `australian-accounting-power-bi`, `payday-super-checker`, `TheExchequerTally`, `SolomonsSword`).

Do not use a redundant H1. Open with a first-person sentence identifying Ryan as an Australian accountant in Newcastle, NSW, then the Hunter Valley market and the review-first position. It must not imply current public-practice employment, registration, vendor affiliation or regulatory endorsement. Keep the US namesake line off this README; it lives on the About page and in `llms.txt`.

The credentials line is an assertion by the profile owner. Confirm it is current before changing or republishing it. Use **provisional member of Chartered Accountants ANZ**, not a vendor-style `CA ANZ` shorthand in prose.

Before the install block, present the three paths: Engage for scoped enquiries, Adopt for supported installs and fabricated-data evaluation, and Verify for credentials, sources and boundaries. Keep one install block near the top with exactly one copy of each primary command: `claude mcp add aus-accounting -- uvx aus-accounting-mcp` for Aus Accounting MCP (`aus-accounting-mcp`) and `npx skills add ryanduguid/australian-accounting-skills` for australian-accounting-skills. The MCP package is published on PyPI; do not replace the package command with a GitHub source install. Link hardhat-ledger later as a specialised workflow without a second `npx` command.

Pins live only in the GitHub UI; the README has no pinned section. The pin record below must match the live list. Keep achievements hidden.

Use a compact architecture flow from ledger data through deterministic rules and agent workflows to human review, with readiness controls also feeding human review. Follow it with a four-row table of representative linked repositories. Keep the full catalogue on the website, not in this README. Do not add an upstream contributions section.

The architecture table is a portfolio map, not a claim that every engine is exposed through Aus Accounting MCP. State its current delegated engines beside the table and verify that list against the server README before publication.

**Engineering boundaries** holds the primary-source, currency-arithmetic, client-data and professional-judgement boundaries. The architecture table carries the reconciliation role; the Ozzit paragraph records provenance. Do not remove a boundary merely to shorten the profile. Put the generated ASCII Profile Ledger at the bottom, immediately before the Hermes attribution.

## Pinned repositories

Change pins in the GitHub UI (**Customize your pins**). After saving, check https://github.com/ryanduguid for the heading **Pinned** (not **Popular**).

Live pin order read back on 27 August 2026 immediately before the repository identity cutover:

1. `xero-trial-balance-export`
2. `payday-super-checker`
3. `aus-accounting-mcp`
4. `workpaper-review-gate`

Preserve these four repository objects and their order during the rename. GitHub may update only their displayed names. Their repository node IDs in order are `R_kgDOTp5MxQ`, `R_kgDOTrBaqQ`, `R_kgDOT_n0mw`, and `R_kgDOUDhjgA`. Do not add, remove, save or reorder pins. Keep achievements hidden. `ato-benchmark-compare` remains public and is called by Aus Accounting MCP; it does not need its own pin. Do not pin `.github`, contribution forks, DiogenesLamp, or Resume-Matcher. GitHub About bio is set in the GitHub UI, not in this repository.

GitHub About on the two flagship repositories (description, homepage, topics) is applied from each repo's `docs/DISCOVERY.md` via `scripts/publish-github-about.sh`.

## Claims that must be checked

| Claim | Source of truth |
| --- | --- |
| Identity, location and credentials | Profile-owner assertions. Confirm with the owner before changing or republishing them |
| Nine public-practice workflow skills, plugin and `npx skills` install | `australian-accounting-skills/README.md` and `.claude/skills/*/SKILL.md` (nine folders) |
| Local MCP facade; uvx from PyPI; delegated engines; Div 7A refused; SBR synthetic | `aus-accounting-mcp/README.md` and `DISCLAIMER.md` |
| Experimental payday-super review, possible SG-charge exposure and no ATO-assessment determination | `payday-super-checker/README.md` and `paydaysuper/deadlines.py` |
| 134 LAMBDA functions, native Excel only, no add-ins or macros | `Ozzit/README.md` |
| Xero trial-balance export requires movement and year-to-date balance before writing | `xero-trial-balance-export/README.md`, the balance-check paragraph under Scope and disclaimer |
| Local profit-and-loss comparison against ATO benchmarks, with working shown | `ato-benchmark-compare/README.md`; do not imply ATO endorsement |
| Source-linked LLM operating guide for Australian accounting, tax and BAS work | `DrDebits/README.md`; do not imply certification or endorsement |

## Style

- Australian English (`judgement`, `honouring`, `licence` in prose).
- No em dashes. [#7](https://github.com/ryanduguid/ryanduguid/pull/7) and [#8](https://github.com/ryanduguid/ryanduguid/pull/8) existed to take them out. List separators are a hyphen with spaces (` - `).
- Do not claim current public-practice employment. [#2](https://github.com/ryanduguid/ryanduguid/pull/2) dropped that wording. The tools are for that domain; that is not the same sentence.
- Do not add a visible LinkedIn link while the profile is inactive. [#16](https://github.com/ryanduguid/ryanduguid/pull/16). That includes the GitHub social-account slot (`gh api user/social_accounts`), not only the README. The canonical website JSON-LD and `llms.txt` may identify the hibernated Australian profile at `https://www.linkedin.com/in/ryan-duguid/` solely to distinguish it from the US namesake's unhyphenated profile.
- Narrow provenance: original work vs forks. [#5](https://github.com/ryanduguid/ryanduguid/pull/5), then [#18](https://github.com/ryanduguid/ryanduguid/pull/18).
- Counts of skills and functions are part of the prose. Recheck them before publication. A renamed project (Nabla to Ozzit, [#14](https://github.com/ryanduguid/ryanduguid/pull/14); CharlesHenryWickens back to payday-super-checker; JohnKenley to au-tax-mcp-server) is a README change in the same breath as the repository rename.

## What stays off the profile

The index is accounting automation for Australian practice. Leave off:

- [DiogenesLamp](https://github.com/ryanduguid/DiogenesLamp), a Yupoo catalogue viewer
- `claude-export`, a private settings snapshot (private repository, so a link would 404 for every visitor)
- [ryanduguid/.github](https://github.com/ryanduguid/.github), account-level community health files

Forks used only to send upstream pull requests stay out of the product list.

## Common mistakes

- Describing payday-super output as a compliance, liability or ATO determination. It is an experimental review aid with stated factual limits.
- Calling Ozzit macro-free while dropping the native-Excel or compatibility context from its own README.
- Saying the Xero exporter writes any trial balance. Both movement and year-to-date balances must reconcile before it writes the CSV.
- Listing six of the nine skills and calling it nine. The named set is BAS, FBT, Division 7A, STP, month-end close, year-end workpapers and 13-week cashflow; `xero-exports` and `workpaper-tie-out` are the other two.
- Treating mentions of Xero, the ATO, CA ANZ or SAP as proof of employment, partnership, approval, registration or endorsement.
- Using retired repository names (`CharlesHenryWickens`, `JohnKenley`, `JohnSpenceOgilvy`, `MaryAddisonHamilton`, `ElizabethAnneAlexander`, `RaymondChambers`, `RussellMathews`, `SirArthurFadden`, `SirAlexanderFitzgerald`, `EdwinNixon`, `LouisGoldberg`) in new copy.

CI runs `.github/workflows/link-check.yml`: every link in README.md, llms.txt, SECURITY.md and docs/ must resolve, links must be https, and a `github.com/ryanduguid/...` link that only works through a rename redirect fails the build.
