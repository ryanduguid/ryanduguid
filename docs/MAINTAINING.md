# Maintaining this profile

This repository is the GitHub profile README for [@ryanduguid](https://github.com/ryanduguid). It is documentation only: there is no application to build, test or deploy.

How to report a security concern is in [SECURITY.md](../SECURITY.md). Account-wide contribution, support and issue-form defaults live in [ryanduguid/.github](https://github.com/ryanduguid/.github). Do not add a `CONTRIBUTING.md` here; it would override that default for this repository only.

## Files

| File | Role |
| --- | --- |
| `README.md` | Public profile at github.com/ryanduguid |
| `assets/banner.svg` | Retained source asset. The minimalist README does not render a banner |
| `SECURITY.md` | Reporting policy for this documentation-only repository |
| `LICENSE` | CC BY 4.0 for the profile prose |
| `docs/MAINTAINING.md` | This runbook |

## Updating the public README

The public README is a short factual index. Keep one H1, three H2 sections, ordinary Markdown links and visible text. Do not add metadata, JSON-LD, hidden text, badges, counters, skill bars or keyword lists.

The H1 is **Ryan Duguid: Australian accounting automation**. The opening paragraph identifies Ryan as an Australian accountant in Newcastle, NSW, then names the supported subject areas once. It must not imply current public-practice employment, registration, vendor affiliation or regulatory endorsement.

The credentials line is an assertion by the profile owner. Confirm it is current before changing or republishing it; do not expand it into an employer, partner, registration or endorsement claim.

**Selected work** has exactly the six repositories listed under **Pinned repositories**, in the same order. Each gets one compact factual bullet taken from its own README, source or release evidence. A second sentence is allowed where a material limitation must remain explicit. The profile is the index; each repository remains the detailed source of truth.

**Open-source contributions** names selected merged examples without a total. Run these cross-platform checks to verify that each named contribution is still merged and correctly attributed:

```bash
gh pr view 3727 --repo meltano/sdk --json author,mergedAt,url
gh pr view 20 --repo Matatika/tap-xero --json author,mergedAt,url
gh pr view 737 --repo farag2/Sophia-Script-for-Windows --json author,mergedAt,url
gh search prs --repo openaccountants/openaccountants --author ryanduguid --merged --limit 100 --json number,url,author,repository
```

Use those results to confirm every contribution named in the profile is still merged and correctly attributed. A new merge does not itself require a profile edit.

**Working method** holds the primary-source, reconciliation, professional-judgement, provenance and client-data boundaries. Keep these statements compact, but do not remove a boundary merely to shorten the profile.

Do not restore the banner, install commands, secondary-project catalogue or pull-request-by-pull-request narrative without a new content decision. Repository READMEs and the pinned cards carry that detail.

## Pinned repositories

GitHub has no public pin API (`pinItem` is not on the public Mutation type). The current
live profile pins, in the order returned by GitHub GraphQL as at 21 August 2026, are
listed below. GitHub's signed-out page and GraphQL response are the source of truth;
do not update this section from an unsaved Customize your pins dialog.

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
| Identity, location and credentials | Profile-owner assertions. Confirm with the owner before changing or republishing them |
| Experimental payday-super review, possible SG-charge exposure and no liability or ATO-assessment determination | `payday-super-checker/README.md` and `paydaysuper/deadlines.py` |
| 130 LAMBDA functions, native Excel only, no add-ins or macros | `Ozzit/README.md` |
| Xero trial-balance export requires movement and year-to-date balance before writing | `xero-trial-balance-export/README.md` "Balance gate" and source checks before CSV write |
| Nine Claude Code skills for Australian public-practice workflows | `australian-accounting-skills/.claude/skills/*/SKILL.md` (nine folders) and that repo's README intro |
| Local profit-and-loss comparison against ATO benchmarks, with working shown | `ato-benchmark-compare/README.md`; do not imply ATO endorsement |
| Source-linked LLM operating guide for Australian accounting, tax and BAS work | `DrDebits/README.md`; do not imply certification or endorsement |
| Selected external work is merged and attributable | The linked pull request pages and the four live GitHub CLI checks above |

## Style

- Australian English (`judgement`, `honouring`, `licence` in prose).
- No em dashes. [#7](https://github.com/ryanduguid/ryanduguid/pull/7) and [#8](https://github.com/ryanduguid/ryanduguid/pull/8) existed to take them out. List separators are a hyphen with spaces (` - `).
- Do not claim current public-practice employment. [#2](https://github.com/ryanduguid/ryanduguid/pull/2) dropped that wording. The tools are for that domain; that is not the same sentence.
- Do not add a LinkedIn link while the profile is inactive. [#16](https://github.com/ryanduguid/ryanduguid/pull/16). That includes the GitHub social-account slot (`gh api user/social_accounts`), not only the README.
- Narrow provenance: original work vs forks. [#5](https://github.com/ryanduguid/ryanduguid/pull/5), then [#18](https://github.com/ryanduguid/ryanduguid/pull/18).
- Counts of skills and functions are part of the prose. Recheck them before publication. A renamed project (Nabla to Ozzit, [#14](https://github.com/ryanduguid/ryanduguid/pull/14)) is a README change in the same breath as the repository rename.

## What stays off the profile

The index is accounting automation for Australian practice. Leave off:

- [DiogenesLamp](https://github.com/ryanduguid/DiogenesLamp), a Yupoo catalogue viewer
- [claude-export](https://github.com/ryanduguid/claude-export), a private settings snapshot
- [ryanduguid/.github](https://github.com/ryanduguid/.github), account-level community health files

Forks used only to send upstream pull requests stay out of **Selected work**. **Open-source contributions** may link merged upstream work, while **Working method** carries the authorship and licence boundary.

## Common mistakes

- Describing payday-super output as a compliance, liability or ATO determination. It is an experimental review aid with stated factual limits.
- Calling Ozzit macro-free while dropping the native-Excel or compatibility context from its own README.
- Saying the Xero exporter writes any trial balance. Both movement and year-to-date balances must reconcile before it writes the CSV.
- Listing six of the nine skills and calling it nine. The named set in the skills README is BAS, FBT, Division 7A, STP, month-end close, year-end workpapers and 13-week cashflow; `xero-exports` and `workpaper-tie-out` are the other two.
- Treating mentions of Xero, the ATO, CA ANZ or SAP as proof of employment, partnership, approval, registration or endorsement.
- Reintroducing a project catalogue, install guide or pull-request inventory. Those details belong in the linked repositories and GitHub's own contribution views.
