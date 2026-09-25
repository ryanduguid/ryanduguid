# Ryan Duguid

I'm a Senior Accountant in Newcastle, Australia. I build open-source tools for cash-flow modelling, month-end review and Australian tax and payroll calculations.

[Website](https://duguid.com.au/) · [Tools](https://duguid.com.au/tools/) · [Evaluations](https://duguid.com.au/evaluate/) · [Credentials and evidence](https://duguid.com.au/evidence/)

I am not a registered tax agent or BAS agent. This is a free software portfolio. I do not accept advice, return-preparation, tax-treatment confirmation, or lodgement requests through it. Support covers software issues reproduced with fabricated data, so please do not send taxpayer information.

## Try an accounting task

| Your task | Example and expected result | What you need |
| --- | --- | --- |
| Explain profit and cash differences | [Fictional Newcastle cash-flow case](https://duguid.com.au/examples/profit-vs-cash-flow/): $35,957.55 quarterly profit and a $25,160 cash shortfall | Read in your browser; desktop Excel to change the receipt delay |
| Calculate in Excel | [Ozzit formula and workbook](https://duguid.com.au/tools/ozzit/#worked-example): extract $100 GST from a wholly taxable $1,100 amount | Microsoft 365 or Excel 2024 or later |
| Review a month-end close | [Monthly Close Controls](https://duguid.com.au/tools/monthly-close-controls/#worked-example): eight exceptions, including a $250 creditors difference | Read in your browser; Git and Python 3.10 or later to reproduce the pack |

Each example includes its working, version and review boundary.

<p align="center"><a href="https://github.com/ryanduguid/llm-tax-guardrails"><img src="https://img.shields.io/badge/APES%20110-Aligned%20Guardrails-4F485E?labelColor=04001F" alt="llm-tax-guardrails: APES 110 aligned guardrails" /></a> <a href="https://duguid.com.au/"><img src="https://img.shields.io/badge/Australian%20Accounting-Open%20Source-5C2D91?labelColor=04001F" alt="Open-source Australian accounting tools" /></a>
  <a href="https://github.com/ryanduguid/Ozzit"><img src="https://img.shields.io/badge/Excel-133%20Native%20LAMBDAs-5C2D91?labelColor=04001F" alt="Ozzit: 133 native Excel LAMBDAs" /></a> <a href="https://www.python.org/"><img src="https://img.shields.io/badge/Python-3.10+-5C2D91?logo=python&logoColor=white&labelColor=04001F" alt="Python 3.10 or later" /></a><br />
  <a href="https://modelcontextprotocol.io/"><img src="https://img.shields.io/badge/MCP-Standard%20Protocol-5C2D91?labelColor=04001F" alt="Model Context Protocol" /></a>
  <a href="https://pypi.org/project/aus-accounting-mcp/"><img src="https://img.shields.io/pypi/v/aus-accounting-mcp?label=PyPI&color=5C2D91&labelColor=04001F" alt="aus-accounting-mcp on PyPI" /></a> <a href="https://github.com/ryanduguid/Ozzit/releases/latest"><img src="https://img.shields.io/github/v/release/ryanduguid/Ozzit?label=Ozzit&color=5C2D91&labelColor=04001F" alt="Latest Ozzit release" /></a></p>

## Selected work

- **[au-fpa-pack](https://github.com/ryanduguid/au-fpa-pack):** Australian cash-flow forecasts and management briefings using fictional businesses. Extends Guiderail's [openfpa](https://github.com/JeffBrines/openfpa).
- **[Accounting Review Pipeline](https://github.com/ryanduguid/accounting-review-pipeline):** Xero exports, month-end exceptions and workpaper review packs, with Excel and Power BI components.
- **[Ozzit](https://github.com/ryanduguid/Ozzit):** 133 native Excel LAMBDA functions plus 5 help tables for financial modelling and GST arithmetic, with editable examples. No macros or add-ins; needs Microsoft 365 or Excel 2024 and later. [What it covers and what it needs](https://duguid.com.au/tools/ozzit/).
- **[Australian Accounting](https://github.com/ryanduguid/australian-accounting):** Tax and payroll calculation packages, plus a local Model Context Protocol (MCP) server for supported tools. Install it with `uvx aus-accounting-mcp`.
- **[Australian Accounting Skills](https://github.com/ryanduguid/australian-accounting-skills):** AI-assisted preparation workflows for public practice and subcontractor accounting: 61 workflows on the default branch ahead of v0.3.0, and 19 in the v0.2.1 release. Recorded model runs cover 17 of its 63 validation cards.
- **[llm-tax-guardrails](https://github.com/ryanduguid/llm-tax-guardrails):** APES 110 and TPB Code controls, refusal patterns and evaluation fixtures for firms using LLMs in tax work. Conclusions stay with the registered practitioner.
- **[au-tax-legislation-corpus](https://github.com/ryanduguid/au-tax-legislation-corpus):** Builds retrieval material from in-force Commonwealth tax legislation on the Federal Register of Legislation, with source and provenance records. It is a finding aid, not authorised legislation.

Public examples use fabricated data, and the tools are preparation and review aids. They do not lodge or write to ledgers; professional judgement and sign-off stay with the reviewer.

<details>
<summary>How the accounting tools fit together</summary>

Skills guide preparation. A configured assistant can call the local MCP server, which delegates calculations to independently released packages. Workpapers and unresolved exceptions go to an authorised human for review.

```mermaid
flowchart TB
    assistant["Assistant using<br/>accounting skills"] <--> mcp["Australian Accounting<br/>MCP server"]
    mcp -->|Calculations| engines["Calculation packages"]
    assistant -->|Workpapers and exceptions| reviewer["Authorised human review"]
    official["Federal Register<br/>of Legislation"] -->|Corpus build| corpus["Legislation corpus<br/>Finding aid"]
    corpus -. Locate provisions .-> reviewer
    official -. Check authority .-> reviewer
```

The corpus is not an automatic source feed into the calculation engines. Dashed arrows show reference use, which still requires checking the applicable authority.

Inspect the [local MCP server](https://github.com/ryanduguid/australian-accounting/tree/main/apps/aus-accounting-mcp), its [PyPI distribution](https://pypi.org/project/aus-accounting-mcp/) and its [MCP Registry listing](https://registry.modelcontextprotocol.io/v0.1/servers/io.github.ryanduguid%2Faus-accounting/versions/latest).

</details>

## Background

- Provisional member of Chartered Accountants ANZ
- SAP S/4HANA certified in [Financial Accounting (FI)](https://www.credly.com/badges/750e7557-ab6d-4b28-a241-8252c263613a/public_url) and [Management Accounting (CO)](https://www.credly.com/badges/0f753c71-5f49-41be-8519-51e81030a8f1/public_url)
- Xero Certified Specialist, Level 3, awarded 1 July 2026 and valid until 1 July 2027

<p><img src="assets/xero-certified-specialist-level-3-badge.png" alt="" width="48" height="48" align="middle" /> Read the <a href="https://duguid.com.au/evidence/#xero-certification">certificate in the evidence register</a>, or explore my <a href="https://duguid.com.au/tools/xero-trial-balance/">Xero trial balance export and review workflow</a>.</p>

<sub>The badge is Xero's artwork, taken from that certificate. Xero has not endorsed or certified anything here.</sub>

To get in touch, use the [contact page](https://duguid.com.au/contact/).
