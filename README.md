# Ryan Duguid: accounting automation & AI agents

Accountant in Newcastle, NSW. CA ANZ Provisional Member · SAP S/4HANA certified in Financial Accounting and Management Accounting · L3 Xero certified specialist.

I build reliable, reviewable tools for Australian accounting work: AI-agent workflows, Power Query parsers, and Xero/API exports. Some encode workflows I know well: month-end closes, BAS quarters, FBT year-ends. Others are design explorations that run on fabricated data only, and say so. All of it is written independently, in my own time and on my own equipment, with no client data.

[LinkedIn](https://www.linkedin.com/in/ryan-duguid/)

## Start here

- Need a payroll control? Start with [payday-super-checker](https://github.com/ryanduguid/payday-super-checker).
- Need an auditable Xero-to-CSV export? Start with [xero-trial-balance-export](https://github.com/ryanduguid/xero-trial-balance-export).
- Exploring AI-assisted accounting workflows? Start with [australian-accounting-skills](https://github.com/ryanduguid/australian-accounting-skills).
- Need a deterministic month-end exception pack? Start with [monthly-close-control-plane](https://github.com/ryanduguid/monthly-close-control-plane).

## Featured projects

### [australian-accounting-skills](https://github.com/ryanduguid/australian-accounting-skills)

AI-agent skills for Australian public-practice workflows: BAS preparation, FBT year-end, Division 7A registers, STP finalisation, month-end close, and workpaper tie-outs. The skills encode the workflow and tie-out discipline, and send the agent to ato.gov.au for information that changes yearly. **Claude Code, SKILL.md, AU tax workflow.**

### [payday-super-checker](https://github.com/ryanduguid/payday-super-checker)

A Python command-line checker for Australian payday-super deadlines. It reads payroll or clearing-house CSVs, flags late and unpaid contributions, and estimates SG-charge exposure with the assumptions visible. **Python, payroll CSV, superannuation compliance.**

### [xero-trial-balance-export](https://github.com/ryanduguid/xero-trial-balance-export)

A Xero trial balance to Power BI-ready CSV in one command. Readable Python that shows every OAuth2 step, survives Xero's single-use refresh-token rotation, and refuses any export where debits fail to equal credits. Replaces a 10-minute manual export-and-clean per entity per month. **Python, OAuth2, Xero API, Power BI.**

### [accounting-excel-toolkit](https://github.com/ryanduguid/accounting-excel-toolkit)

Power Query functions and VBA modules for real month-end work: a Xero trial balance parser, an ABN checksum validator, an AU financial-year helper, and a keyed reconciliation engine for subledger-vs-GL differences. All source lives as text, never buried in a binary workbook. **Power Query M, VBA, Excel.**

### [au-tax-legislation-corpus](https://github.com/ryanduguid/au-tax-legislation-corpus)

A reproducible builder for a provenance-rich retrieval corpus of in-force Commonwealth tax legislation. It keeps source, version, licensing and distribution controls alongside the data pipeline. **Python, legal data, RAG.**

### [monthly-close-control-plane](https://github.com/ryanduguid/monthly-close-control-plane)

A deterministic, review-first control pack for current/prior trial balances, variance thresholds, mapping coverage and subledger reconciliations. It produces evidence for human review without journals, payments or period-locking actions. **Python, month-end close, controls.**

### [xero-ai-review-gateway](https://github.com/ryanduguid/xero-ai-review-gateway)

A synthetic-data reference boundary for fixed-policy, redacted trial-balance review. It separates model-facing findings from reviewer evidence and exposes no accounting mutation tools. **Python, Xero-shaped data, AI governance.**

### [au-tax-change-impact-monitor](https://github.com/ryanduguid/au-tax-change-impact-monitor)

A provenance-first synthetic change-review queue that preserves source-version and incomplete-scope states, maps only exact source identifiers, and leaves technical-tax decisions to a human reviewer. **Python, tax provenance, review workflow.**

## Selected upstream contributions

Merged contributions to external open-source projects:

- [Meltano SDK: retain returned OAuth refresh tokens (PR #3727)](https://github.com/meltano/sdk/pull/3727)
- [Matatika/tap-xero: correct OAuth configuration examples (PR #20)](https://github.com/Matatika/tap-xero/pull/20)
- [OpenAccountants: payday super and payroll update for 2026-27 (PR #77)](https://github.com/openaccountants/openaccountants/pull/77), [AU FBT guide (PR #78)](https://github.com/openaccountants/openaccountants/pull/78), [AU Div 7A guide (PR #79)](https://github.com/openaccountants/openaccountants/pull/79), and [AU GST/BAS PAYG reporting guidance (PR #84)](https://github.com/openaccountants/openaccountants/pull/84)
- [OpenAccountants: restore the AU GST/BAS guide and add a sync-integrity guard (PR #85)](https://github.com/openaccountants/openaccountants/pull/85): recovers content a website sync had overwritten, and adds a CI check and tests so the same class of loss fails the build
- [OpenAccountants: bind the MCP HTTP transport to loopback by default (PR #89)](https://github.com/openaccountants/openaccountants/pull/89) and [drop an obsolete licence classifier that broke the package build (PR #90)](https://github.com/openaccountants/openaccountants/pull/90): two fixes to the project's MCP server, each with the failure reproduced first

## How I build

- **Workflow before content.** Automate repeatable steps and checks; verify rates, dates and legislation against the primary source.
- **Tie out or it didn't happen.** Outputs should reconcile back to their source data and expose the exceptions that need review.
- **Professional judgement stays human.** Tools support review-heavy work; they do not replace professional judgement or client confidentiality obligations.

## Now

Accountant exploring how AI agents can make accounting-firm workflows more repeatable without taking over professional judgement.

## Contact

[LinkedIn](https://www.linkedin.com/in/ryan-duguid/)
