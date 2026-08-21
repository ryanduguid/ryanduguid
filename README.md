![Ryan Duguid, accounting automation and AI agents, Australian practice](assets/banner.svg)

Tools for Australian public-practice accounting: Xero, Excel, and primary-source tax data.

Provisional Member CA ANZ &#183; SAP S/4HANA certified in FI and CO &#183; Xero L3

## Start here

**[payday-super-checker](https://github.com/ryanduguid/payday-super-checker)** - Reads a payroll CSV, checks fund receipt against the ordinary seven-business-day period and supported allowable longer periods, and produces an experimental SG-charge estimate where the supplied facts establish lateness. It does not determine an employer's legal liability or the ATO's assessment. Python 3.10 or later, no runtime dependencies.

```bash
git clone https://github.com/ryanduguid/payday-super-checker.git
cd payday-super-checker && pip install .
payday-super-check examples/sample_payrun_no_transition.csv --as-at 2026-09-10
```

The sample file is in the clone. `pip install git+https://github.com/ryanduguid/payday-super-checker.git` installs the command without it.

**[Ozzit](https://github.com/ryanduguid/Ozzit)** - 130 LAMBDA functions in one workbook for dynamic-array financial models. Built from native Excel functions only, so workbooks you assemble with it save as an ordinary .xlsx with no add-ins and no macros. Needs Microsoft 365 or Excel 2024. [Download ozzit.xlsx](https://github.com/ryanduguid/Ozzit/releases/latest/download/ozzit.xlsx).

**[australian-accounting-skills](https://github.com/ryanduguid/australian-accounting-skills)** - Nine Claude Code skills covering BAS, FBT, Division 7A, STP, month-end close, year-end workpapers and 13-week cashflow.

In Claude Code:

```
/plugin marketplace add ryanduguid/australian-accounting-skills
/plugin install australian-accounting-skills@ryanduguid
```

## Also

- [xero-trial-balance-export](https://github.com/ryanduguid/xero-trial-balance-export) - One command, one balanced CSV. It refuses to write the file when debits do not equal credits.
- [subcontractor-accounting-skills](https://github.com/ryanduguid/subcontractor-accounting-skills) - Agent skills for contracting and mining services. Progress claims, retentions, WIP, fuel tax credits, Coal LSL, the contractor limbs of payroll tax.
- [accounting-excel-toolkit](https://github.com/ryanduguid/accounting-excel-toolkit) - Power Query and VBA that live as text in git, not buried inside a workbook nobody can diff.
- [ato-benchmark-compare](https://github.com/ryanduguid/ato-benchmark-compare) - Scores a profit and loss against the ATO small business benchmarks on your own machine, and shows the working.
- [monthly-close-control-plane](https://github.com/ryanduguid/monthly-close-control-plane) - Builds the exception pack a close review actually needs. It writes no journals and locks no periods.
- [au-tax-legislation-corpus](https://github.com/ryanduguid/au-tax-legislation-corpus) - A reproducible build of in-force Commonwealth tax legislation for retrieval, with the provenance kept next to the text.
- [au-tax-change-impact-monitor](https://github.com/ryanduguid/au-tax-change-impact-monitor) - A review queue for possible changes to tax sources. It matches identifiers and leaves the judgement to a person.
- [xero-ai-review-gateway](https://github.com/ryanduguid/xero-ai-review-gateway) - A boundary experiment on synthetic data for what an AI reviewer may see, and what it may never touch.
- [DrDebits](https://github.com/ryanduguid/DrDebits) - Versioned, source-linked ethics instructions for tax practitioners working with LLMs.
- [release-policy](https://github.com/ryanduguid/release-policy) - The reusable release workflow behind the tags in these repositories. Fail-closed gates, SBOM, build provenance.

## Selected work merged into other people's projects

Most of these changes are small, which is rather the point.
[meltano/sdk#3727](https://github.com/meltano/sdk/pull/3727) keeps a rotated
OAuth refresh token instead of throwing it away.
[Matatika/tap-xero#20](https://github.com/Matatika/tap-xero/pull/20) corrects
configuration examples that could not have worked.
[farag2/Sophia-Script-for-Windows#737](https://github.com/farag2/Sophia-Script-for-Windows/pull/737)
puts copied localization strings back in the language files they belong to. Twelve at
[openaccountants](https://github.com/openaccountants/openaccountants/pulls?q=is%3Apr+author%3Aryanduguid+is%3Amerged):
the Australian super, FBT, Division 7A and GST guides
([#77](https://github.com/openaccountants/openaccountants/pull/77),
[#78](https://github.com/openaccountants/openaccountants/pull/78),
[#79](https://github.com/openaccountants/openaccountants/pull/79),
[#84](https://github.com/openaccountants/openaccountants/pull/84)); restoring a
guide a website sync had overwritten, with a CI check so the same loss
fails the build next time
([#85](https://github.com/openaccountants/openaccountants/pull/85)); binding the
MCP transport to loopback
([#89](https://github.com/openaccountants/openaccountants/pull/89)); dropping
a licence classifier that broke the package build
([#90](https://github.com/openaccountants/openaccountants/pull/90));
honouring explicit skill quality tiers
([#94](https://github.com/openaccountants/openaccountants/pull/94));
failing closed on divergent duplicate slugs
([#97](https://github.com/openaccountants/openaccountants/pull/97));
deriving the contradiction-scan tax year from rates
([#98](https://github.com/openaccountants/openaccountants/pull/98));
validating workbook filters
([#100](https://github.com/openaccountants/openaccountants/pull/100));
and pinning workflow actions and scoping checkout credentials
([#104](https://github.com/openaccountants/openaccountants/pull/104)).

## How this is written

Rates, dates and section numbers get checked against the primary source.
Outputs reconcile back to what they came from and surface whatever does not tie.
The tools prepare evidence and surface exceptions. A suitably authorised person remains responsible for professional judgement and any consequential action.

## This account

I write my original projects in my own time, on my own equipment, using synthetic or non-client data.
Some repositories on this account are forks used for upstream contributions; those forks retain the
upstream projects' authorship and licence histories. No client data is committed.
