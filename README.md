# Ryan Duguid

Accountant in Newcastle NSW. CA ANZ Provisional Member. SAP S/4HANA certified in Financial Accounting and Management Accounting, L3 Xero certified specialist.

I build tools that take the manual work out of accounting: Claude Code skills, Power Query parsers, and API exports. Everything here started as work I did by hand: month-end closes, BAS quarters, FBT year-ends.

## Projects

### [australian-accounting-skills](https://github.com/ryanduguid/australian-accounting-skills)

Claude Code skills for Australian public-practice workflows: BAS preparation, FBT year-end, Division 7A registers, STP finalisation, month-end close, workpaper tie-outs. Agent skill packs existed for US corporate finance and Swedish bookkeeping law; I couldn't find one for AU practice, so I built it. Nine skills encode the workflow and the tie-out discipline, and send the agent to ato.gov.au for anything that changes yearly. **Claude Code, SKILL.md, AU tax workflow.**

### [accounting-excel-toolkit](https://github.com/ryanduguid/accounting-excel-toolkit)

Power Query functions and VBA modules from real month-end work: a Xero trial balance parser that survives the export format's traps, an ABN checksum validator, an AU financial-year helper, and a keyed reconciliation engine for subledger-vs-GL differences. All source lives as text, never buried in a binary workbook. **Power Query M, VBA, Excel.**

### [xero-trial-balance-export](https://github.com/ryanduguid/xero-trial-balance-export)

A trial balance from the Xero API to a Power BI-ready CSV in one command. Raw-requests Python that shows every OAuth2 step, survives Xero's single-use refresh-token rotation, and refuses any export where debits fail to equal credits. Replaces a 10-minute manual export-and-clean per entity per month. **Python, OAuth2, Xero API, Power BI.**

## Now

Public-practice accountant building the case that AI agents belong in accounting firms. Interested in how Claude fits review-heavy workflows without touching professional judgment.

## Contact

[LinkedIn](https://www.linkedin.com/in/ryan-duguid/)
