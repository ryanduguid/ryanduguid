# Maintaining this profile

This repository is the GitHub profile README for [@ryanduguid](https://github.com/ryanduguid). It is documentation only: there is no application to build, test or deploy.

How to report a security concern is in [SECURITY.md](../SECURITY.md). Account-wide contribution, support and issue-form defaults live in [ryanduguid/.github](https://github.com/ryanduguid/.github). Do not add a `CONTRIBUTING.md` here; it would override that default for this repository only.

## Files

| File | Role |
| --- | --- |
| `README.md` | Public profile at github.com/ryanduguid |
| `llms.txt` | Compact agent-facing index; keep it aligned with README |
| `assets/banner.svg` | Retained source asset. The README does not render a banner |
| `SECURITY.md` | Reporting policy for this documentation-only repository |
| `LICENSE` | CC BY 4.0 for the profile prose |
| `docs/MAINTAINING.md` | This runbook |

## Updating the public README

The profile is an index, not a second catalogue of every engine. Lead with the two installable products, then the engines those products call. Keep current repository names (`JohnKenley`, `CharlesHenryWickens`, `TheExchequerTally`, `SolomonsSword`, `RussellMathews`).

Keep one H1 (`Ryan Duguid`). The opening paragraph identifies Ryan as an Australian accountant in Newcastle, NSW, then names the supported subject areas once. It must not imply current public-practice employment, registration, vendor affiliation or regulatory endorsement.

The credentials line is an assertion by the profile owner. Confirm it is current before changing or republishing it. Use **provisional member of Chartered Accountants ANZ**, not a vendor-style `CA ANZ` shorthand in prose.

**Install these two** is MaryAddisonHamilton then JohnKenley, each with the shortest install path that actually works. Do not add a third installable product here without a packaging decision in that repository.

**Featured (pinned)** must match the live GitHub pin list. GitHub has no public pin API.

**Open-source contributions** names selected merged examples without a total. Run these checks to verify each named contribution is still merged and correctly attributed:

```bash
gh pr view 3727 --repo meltano/sdk --json author,mergedAt,url
gh pr view 20 --repo Matatika/tap-xero --json author,mergedAt,url
gh pr view 737 --repo farag2/Sophia-Script-for-Windows --json author,mergedAt,url
gh search prs --repo openaccountants/openaccountants --author ryanduguid --merged --limit 100 --json number,url,author,repository
```

**Working method** holds the primary-source, reconciliation, professional-judgement, provenance and client-data boundaries. Do not remove a boundary merely to shorten the profile.

## Pinned repositories

Change pins in the GitHub UI (**Customize your pins**). After saving, check https://github.com/ryanduguid for the heading **Pinned** (not **Popular**).

Live pin order as at 21 August 2026:

1. `CharlesHenryWickens` (formerly `payday-super-checker`)
2. `Ozzit`
3. `JohnSpenceOgilvy`
4. `MaryAddisonHamilton`
5. `RaymondChambers`
6. `DrDebits`

Intended next pin order, once changed in the UI:

1. `MaryAddisonHamilton`
2. `JohnKenley` (formerly `aus-accounting-mcp`)
3. `CharlesHenryWickens`
4. `Ozzit`
5. `JohnSpenceOgilvy`
6. `DrDebits`

`RaymondChambers` remains public and is called by `JohnKenley`; it does not need its own pin once the MCP server is pinned. Do not pin `.github`, contribution forks, or Resume-Matcher.

GitHub About on the two flagship repositories (description, homepage, topics) is applied from each repo's `docs/DISCOVERY.md` via `scripts/publish-github-about.sh`.

## Claims that must be checked

| Claim | Source of truth |
| --- | --- |
| Identity, location and credentials | Profile-owner assertions. Confirm with the owner before changing or republishing them |
| Nine public-practice workflow skills, plugin and `npx skills` install | `MaryAddisonHamilton/README.md` and `.claude/skills/*/SKILL.md` (nine folders) |
| Local MCP facade; uvx from GitHub; Div 7A refused; SBR synthetic | `JohnKenley/README.md` and `DISCLAIMER.md` |
| Experimental payday-super review, possible SG-charge exposure and no ATO-assessment determination | `CharlesHenryWickens/README.md` and `paydaysuper/deadlines.py` |
| 130 LAMBDA functions, native Excel only, no add-ins or macros | `Ozzit/README.md` |
| Xero trial-balance export requires movement and year-to-date balance before writing | `JohnSpenceOgilvy/README.md` "Balance gate" |
| Local profit-and-loss comparison against ATO benchmarks, with working shown | `RaymondChambers/README.md`; do not imply ATO endorsement |
| Source-linked LLM operating guide for Australian accounting, tax and BAS work | `DrDebits/README.md`; do not imply certification or endorsement |
| Selected external work is merged and attributable | The linked pull request pages and the four live GitHub CLI checks above |

## Style

- Australian English (`judgement`, `honouring`, `licence` in prose).
- No em dashes. [#7](https://github.com/ryanduguid/ryanduguid/pull/7) and [#8](https://github.com/ryanduguid/ryanduguid/pull/8) existed to take them out. List separators are a hyphen with spaces (` - `).
- Do not claim current public-practice employment. [#2](https://github.com/ryanduguid/ryanduguid/pull/2) dropped that wording. The tools are for that domain; that is not the same sentence.
- Do not add a LinkedIn link while the profile is inactive. [#16](https://github.com/ryanduguid/ryanduguid/pull/16). That includes the GitHub social-account slot (`gh api user/social_accounts`), not only the README.
- Narrow provenance: original work vs forks. [#5](https://github.com/ryanduguid/ryanduguid/pull/5), then [#18](https://github.com/ryanduguid/ryanduguid/pull/18).
- Counts of skills and functions are part of the prose. Recheck them before publication. A renamed project (Nabla to Ozzit, [#14](https://github.com/ryanduguid/ryanduguid/pull/14); payday-super-checker to CharlesHenryWickens; aus-accounting-mcp to JohnKenley) is a README change in the same breath as the repository rename.

## What stays off the profile

The index is accounting automation for Australian practice. Leave off:

- [DiogenesLamp](https://github.com/ryanduguid/DiogenesLamp), a Yupoo catalogue viewer
- [claude-export](https://github.com/ryanduguid/claude-export), a private settings snapshot
- [ryanduguid/.github](https://github.com/ryanduguid/.github), account-level community health files

Forks used only to send upstream pull requests stay out of the product list. **Open-source contributions** may link merged upstream work.

## Common mistakes

- Describing payday-super output as a compliance, liability or ATO determination. It is an experimental review aid with stated factual limits.
- Calling Ozzit macro-free while dropping the native-Excel or compatibility context from its own README.
- Saying the Xero exporter writes any trial balance. Both movement and year-to-date balances must reconcile before it writes the CSV.
- Listing six of the nine skills and calling it nine. The named set is BAS, FBT, Division 7A, STP, month-end close, year-end workpapers and 13-week cashflow; `xero-exports` and `workpaper-tie-out` are the other two.
- Treating mentions of Xero, the ATO, CA ANZ or SAP as proof of employment, partnership, approval, registration or endorsement.
- Using retired repository names (`aus-accounting-mcp`, `payday-super-checker`, `EdwinNixon`, `LouisGoldberg`) in new copy.

CI now runs `.github/workflows/link-check.yml` so profile README links stay https.
