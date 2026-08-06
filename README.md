# Ryan Duguid — Accounting automation & AI agents

Public-practice accountant in Newcastle, NSW. CA ANZ Provisional Member · SAP S/4HANA certified in Financial Accounting and Management Accounting · L3 Xero certified specialist.

I build reliable, reviewable tools for Australian accounting work: AI-agent workflows, Power Query parsers, and Xero/API exports. Everything here started as work I did by hand—month-end closes, BAS quarters, and FBT year-ends.

[LinkedIn](https://www.linkedin.com/in/ryan-duguid/)

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

## How I build

- **Workflow before content.** Automate repeatable steps and checks; verify rates, dates and legislation against the primary source.
- **Tie out or it didn't happen.** Outputs should reconcile back to their source data and expose the exceptions that need review.
- **Professional judgement stays human.** Tools support review-heavy work; they do not replace professional judgement or client confidentiality obligations.

## Now

Public-practice accountant exploring how AI agents can make accounting-firm workflows more repeatable without taking over professional judgement.

## Contact

[LinkedIn](https://www.linkedin.com/in/ryan-duguid/)
