# Ryan Duguid

<p align="center">
  <img src="https://img.shields.io/badge/Provisional_Member-CA_ANZ-5C2D91?style=for-the-badge&labelColor=04001F" alt="CA ANZ" />
  <img src="https://img.shields.io/badge/SAP_Certified-S%2F4HANA_FI%2FCO-5C2D91?style=for-the-badge&labelColor=04001F" alt="SAP FI/CO" />
  <img src="https://img.shields.io/badge/Xero-L3_Specialist-5C2D91?style=for-the-badge&logo=xero&logoColor=white&labelColor=04001F" alt="Xero" />
</p>

<p align="center">
  <a href="https://www.python.org/"><img src="https://img.shields.io/badge/Python-3.10+-5C2D91?logo=python&logoColor=white&labelColor=04001F" alt="Python 3.10 or later" /></a>
  <a href="https://modelcontextprotocol.io/"><img src="https://img.shields.io/badge/MCP-Standard%20Protocol-5C2D91?labelColor=04001F" alt="Model Context Protocol" /></a>
  <a href="https://github.com/ryanduguid/Ozzit"><img src="https://img.shields.io/badge/Excel-130%20Native%20LAMBDAs-5C2D91?logo=microsoftexcel&logoColor=white&labelColor=04001F" alt="Ozzit: 130 native Excel LAMBDAs" /></a>
  <a href="https://github.com/ryanduguid/DrDebits"><img src="https://img.shields.io/badge/APES%20110-Compliant%20Guardrails-4F485E?labelColor=04001F" alt="DrDebits APES 110 guardrails" /></a>
  <a href="https://github.com/ryanduguid/MaryAddisonHamilton"><img src="https://img.shields.io/badge/Claude_Code-Skills%20%26%20Agents-5C2D91?logo=anthropic&logoColor=white&labelColor=04001F" alt="Mary Addison Hamilton Claude Code skills" /></a>
</p>

---

Australian accountant based in Newcastle, NSW. Engineering **computational accounting engines**, **native Excel dynamic-array LAMBDAs**, **Model Context Protocol (MCP) servers**, and **deterministic AI workflows** for Australian taxation, payroll, and statutory compliance.

> **Provisional Member CA ANZ** &bull; **SAP S/4HANA Certified in FI & CO** &bull; **L3 Xero Specialist Certified**

---

## ⚡ 1-Command Agent Toolbelt

Equip your AI assistant with Australian taxation rules and computational accounting engines:

```bash
# Add Australian Accounting & Tax MCP server to Claude Code
claude mcp add aus-accounting -- uvx --from git+https://github.com/ryanduguid/JohnKenley aus-accounting-mcp

# Add Public Practice & Tax skills
npx skills add ryanduguid/MaryAddisonHamilton

# Add Construction & Mining Subcontractor skills
npx skills add ryanduguid/hardhat-ledger
```

---

## The stack those products sit on

```mermaid
%%{init: {
  'theme': 'base',
  'themeVariables': {
    'primaryTextColor': '#FFFFFF',
    'lineColor': '#B1AFAD',
    'tertiaryColor': '#04001F'
  }
}}%%
flowchart LR
    subgraph Data ["1. Data and ledgers"]
        direction TB
        Xero["JohnSpenceOgilvy<br/><i>Validated Xero TB Export</i>"]
        PQ["SirAlexanderFitzgerald<br/><i>Power Query (M) & VBA</i>"]
    end

    subgraph Engines ["2. Rules and engines"]
        direction TB
        Ozzit["Ozzit<br/><i>130 Native Excel LAMBDAs</i>"]
        Tally["TheExchequerTally<br/><i>Company Tax & Franking FAB</i>"]
        Sword["SolomonsSword<br/><i>Trust Div 6 & s100A Matrix</i>"]
        ATO["RaymondChambers<br/><i>ATO Benchmark Compare</i>"]
        Super["CharlesHenryWickens<br/><i>Payday Super & SG Charge</i>"]
    end

    subgraph AI ["3. Agent workflows"]
        direction TB
        MCP["JohnKenley<br/><i>Unified MCP Server</i>"]
        MAH["MaryAddisonHamilton<br/><i>Public-Practice Skills</i>"]
        SubSkills["Hardhat Ledger<br/><i>Construction & Mining Skills</i>"]
        DrD["DrDebits<br/><i>APES 110 & TPB Ethics Gate</i>"]
        Gateway["ElizabethAnneAlexander<br/><i>Zero-Network Review Sandbox</i>"]
    end

    Data --> Engines --> AI

    classDef dataBox fill:#140E24,stroke:#4F485E,stroke-width:1.5px,color:#FFFFFF;
    classDef engineBox fill:#1E1236,stroke:#5C2D91,stroke-width:1.5px,color:#FFFFFF;
    classDef aiBox fill:#2D184E,stroke:#8A4AC7,stroke-width:1.5px,color:#FFFFFF;

    class Xero,PQ dataBox;
    class Ozzit,Tally,Sword,ATO,Super engineBox;
    class MCP,MAH,SubSkills,DrD,Gateway aiBox;
```

JohnKenley calls CharlesHenryWickens and RaymondChambers (`ato-benchmark-compare`). MaryAddisonHamilton names those CLIs rather than inventing the same work. Historical engine repositories stay the source of truth for the calculation; the two products above are what a stranger should install.

---

## 📦 Categorized Open-Source Ecosystem

### 🔌 Model Context Protocol (MCP) & Agent Infrastructure
- **[JohnKenley](https://github.com/ryanduguid/JohnKenley)** (`aus-accounting-mcp`) - Unified Model Context Protocol (MCP) server exposing ATO small business benchmarks, Payday Super statutory timelines, and synthetic SBR test payloads to Claude Desktop, Claude Code, Cursor, and Antigravity.

### 🧮 Computational Engines & Statutory Rules
- **[Ozzit](https://github.com/ryanduguid/Ozzit)** - Comprehensive library of 130 native Excel `LAMBDA` functions for dynamic-array financial modelling, loan amortisation, capital allowance depreciation, and deterministic cash flow forecasting without VBA.
- **[TheExchequerTally](https://github.com/ryanduguid/TheExchequerTally)** *(Corporate Tax & Franking)* - Corporate tax rate determination (Base Rate Entity eligibility under *s 23AA ITRA 1986*), Franking Account Balance (FAB) tracking, and Division 203 benchmark rule compliance.
- **[SolomonsSword](https://github.com/ryanduguid/SolomonsSword)** *(Trust Income & Section 100A)* - Trust income allocation under Division 6 ITAA 1936 (*Bamford* proportionate approach), Section 100A risk classification (*ATO PCG 2022/2*), and Section 99B foreign trust receipt analysis.
- **[CharlesHenryWickens](https://github.com/ryanduguid/CharlesHenryWickens)** (`payday-super-checker`) - Deterministic engine evaluating Australian Payday Super 2026 contribution timelines and Superannuation Guarantee (SG) charge exposure on late remittances.
- **[RaymondChambers](https://github.com/ryanduguid/RaymondChambers)** *(`ato-benchmark-compare`)* - Localized, offline profit-and-loss variance analysis against Australian Taxation Office (ATO) small business benchmarks.

### 🛡️ AI Agent Workflows & Deterministic Safety Boundaries
- **[DrDebits](https://github.com/ryanduguid/DrDebits)** - Primary-source-grounded ethical guardrails (aligned with APES 110 and TPB Code of Professional Conduct) for LLM-assisted taxation workflows.
- **[ElizabethAnneAlexander](https://github.com/ryanduguid/ElizabethAnneAlexander)** - Zero-network, fixed-policy safety boundary for AI-assisted trial balance review with cryptographic receipt verification and data minimisation.
- **[MaryAddisonHamilton](https://github.com/ryanduguid/MaryAddisonHamilton)** - Nine modular agent skills for Australian public practice (BAS reconciliation, FBT, Division 7A benchmark compliance, STP Phase 2, and 13-week cash flow modelling).
- **[Hardhat Ledger](https://github.com/ryanduguid/hardhat-ledger)** - Claude Code skills for Australian construction and mining subcontractors (Security of Payment claims, retentions, WIP, Coal LSL, and Fuel Tax Credits).

### 📊 Ledger Controls & Pipeline Utilities
- **[JohnSpenceOgilvy](https://github.com/ryanduguid/JohnSpenceOgilvy)** *(`xero-trial-balance-export`)* - Exports reconciled Xero trial balances to validated CSV formats with strict movement and year-to-date mathematical integrity.
- **[SirAlexanderFitzgerald](https://github.com/ryanduguid/SirAlexanderFitzgerald)** - Power Query (M) and VBA automation utilities for Australian accounting, reconciliations, and month-end financial reporting pipelines.
- **[awesome-australian-accounting-tech](https://github.com/ryanduguid/awesome-australian-accounting-tech)** - A curated index of open-source libraries, computational tax engines, ATO datasets, and Commonwealth legislation APIs.

---

## 🤝 Upstream Open-Source Contributions

- **[Meltano SDK #3727](https://github.com/meltano/sdk/pull/3727)** - Preservation of OAuth refresh tokens returned by token refresh endpoints.
- **[tap-xero #20](https://github.com/Matatika/tap-xero/pull/20)** - Rectification of OAuth 2.0 configuration specifications and schema definitions.
- **[OpenAccountants](https://github.com/openaccountants/openaccountants/pulls?q=is%3Apr+author%3Aryanduguid+is%3Amerged)** - Australian taxation guidance, validation test harnesses, and CI automation for AI agent tax guides.
- **[Sophia Script for Windows #737](https://github.com/farag2/Sophia-Script-for-Windows/pull/737)** - Localisation and language architecture enhancements.

---

## 📐 Engineering Principles

1. **Primary Source Grounding**: All statutory computations (marginal tax brackets, SG percentages, benchmark ratios) are grounded directly in Commonwealth primary legislation (*ITAA 1936/1997*, *SGAA 1992*), ATO public rulings, and AASB/APESB standards.
2. **Exact Decimal Arithmetic**: Computational outputs use exact decimal quantization to prevent binary floating-point drift.
3. **Local Privacy Boundaries**: Client-sensitive financial data remains in local memory or zero-network sandboxes; public fixtures use synthetic data only.
4. **Human-in-the-Loop Signoff**: Algorithmic pipelines automate calculation and structural validation; ultimate professional judgement and statutory lodgements remain strictly human-controlled.
