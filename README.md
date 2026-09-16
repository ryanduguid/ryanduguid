# Ryan Duguid

I'm an accountant in Newcastle, Australia. I build open-source controls for Australian tax, payroll and financial reporting.

Start with the [fictional Newcastle cash-flow case](https://duguid.com.au/evaluate/#profit-and-cash): an Excel forecast, a management briefing, and a late-payment scenario.

<p align="center"><a href="https://github.com/ryanduguid/llm-tax-guardrails"><img src="https://img.shields.io/badge/APES%20110-Aligned%20Guardrails-4F485E?labelColor=04001F" alt="llm-tax-guardrails: APES 110 aligned guardrails" /></a> <a href="https://duguid.com.au/"><img src="https://img.shields.io/badge/Australian%20Accounting-Open%20Source-5C2D91?labelColor=04001F" alt="Open-source Australian accounting tools" /></a>
  <a href="https://github.com/ryanduguid/Ozzit"><img src="https://img.shields.io/badge/Excel-133%20Native%20LAMBDAs-5C2D91?labelColor=04001F" alt="Ozzit: 133 native Excel LAMBDAs" /></a> <a href="https://www.python.org/"><img src="https://img.shields.io/badge/Python-3.10+-5C2D91?logo=python&logoColor=white&labelColor=04001F" alt="Python 3.10 or later" /></a>
  <a href="https://modelcontextprotocol.io/"><img src="https://img.shields.io/badge/MCP-Standard%20Protocol-5C2D91?labelColor=04001F" alt="Model Context Protocol" /></a></p>

## Selected work

- [au-fpa-pack](https://github.com/ryanduguid/au-fpa-pack) explains profit and cash through a fictional Newcastle maintenance business, with an Excel forecast, a management briefing and a scenario where a customer pays late.
- [australian-accounting](https://github.com/ryanduguid/australian-accounting) brings the Australian tax and payroll engines together with the local MCP server.
- [accounting-review-pipeline](https://github.com/ryanduguid/accounting-review-pipeline) connects Xero exports, close controls and review packs with Excel and Power BI.
- [australian-accounting-skills](https://github.com/ryanduguid/australian-accounting-skills) provides preparation workflows for public practice and subcontractor accounting: 19 in release v0.2.1 and 50 on the default branch preparing v0.3.0.
- [Ozzit](https://github.com/ryanduguid/Ozzit) provides 133 native Excel LAMBDA functions plus 5 help tables for financial modelling and GST arithmetic.

Accountants can [browse the tools](https://duguid.com.au/tools/), developers can inspect the [local MCP server](https://github.com/ryanduguid/australian-accounting/tree/main/apps/aus-accounting-mcp), and reviewers can [reproduce the public evaluations](https://duguid.com.au/evaluate/) using fabricated inputs.

The projects use synthetic public examples and support professional review. They do not lodge or write to ledgers.

## How the accounting tools fit together

Skills guide preparation. A configured assistant can call the local MCP server,
which delegates calculations to independently released packages. Workpapers and
unresolved exceptions go to an authorised human for review.

```mermaid
flowchart TB
    assistant["Assistant using<br/>accounting skills"] <--> mcp["Aus Accounting MCP"]
    mcp -->|Calculations| engines["Calculation packages"]
    assistant -->|Workpapers and exceptions| reviewer["Authorised human review"]
    official["Federal Register"] -->|Corpus build| corpus["Legislation corpus<br/>Finding aid"]
    corpus -. Locate provisions .-> reviewer
    official -. Check authority .-> reviewer
```

The [corpus](https://github.com/ryanduguid/au-tax-legislation-corpus) is a separate
finding aid derived from the Register's EPUB reading view. It is not authorised
legislation or an automatic source feed into the calculation engines. Dashed
arrows show reference use, which still requires checking the applicable authority.

## Background

- Provisional member of Chartered Accountants ANZ
- SAP S/4HANA certified in [FI](https://www.credly.com/badges/750e7557-ab6d-4b28-a241-8252c263613a/public_url) and [CO](https://www.credly.com/badges/0f753c71-5f49-41be-8519-51e81030a8f1/public_url)
- Xero Certified Specialist, Level 3, awarded 1 July 2026 and valid until 1 July 2027

<p><img src="assets/xero-certified-specialist-level-3-badge.png" alt="" width="48" height="48" align="middle" /> Read the <a href="https://duguid.com.au/evidence/#xero-certification">certificate in the evidence register</a>, or explore my <a href="https://duguid.com.au/tools/xero-trial-balance/">Xero trial balance export and review workflow</a>.</p>

<sub>The badge is Xero's artwork, taken from that certificate. Xero has not endorsed or certified anything here.</sub>

I publish [Aus Accounting MCP](https://registry.modelcontextprotocol.io/v0.1/servers/io.github.ryanduguid%2Faus-accounting/versions/latest) on [PyPI](https://pypi.org/project/aus-accounting-mcp/). Credentials, release provenance, and test evidence are on the [evidence page](https://duguid.com.au/evidence/).

## Contributions

Across 15 upstream projects I have [49 merged pull requests](https://github.com/search?q=is%3Apr+author%3Aryanduguid+is%3Amerged+merged%3A%3C%3D2026-09-14+-user%3Aryanduguid&type=pullrequests) as at 14 September 2026, 33 of them in [OpenAccountants](https://github.com/OpenAccountants/openaccountants/pulls?q=is%3Apr+author%3Aryanduguid+is%3Amerged+merged%3A%3C%3D2026-09-14). Two examples:

- **Meltano SDK:** I fixed OAuth refresh-token handling so the authenticator keeps a replacement token and preserves the existing token when no replacement arrives. The change includes regression tests and updates the in-memory authenticator. Persistent configuration write-back is outside its scope. [Merged 8 August 2026](https://github.com/meltano/sdk/pull/3727).
- **OpenAccountants:** I restored Australian BAS guide corrections after an older platform export overwrote them, and added checks for stale source changes. The repository checks detect this overwrite pattern. Preventing it also requires changes to the private exporter. [Merged 11 August 2026](https://github.com/OpenAccountants/openaccountants/pull/85).

## Setup

OpenHands runs on my old uni laptop with CachyOS, unattended, so its commit timestamps show when a job finished rather than when I was at a keyboard. I use Hermes Agent on my Windows 11 IoT Enterprise LTSC desktop and supplement my vitamin D. Both machines are my own and everything here is built in my own time.
