# Forks

Tracking-only forks. No Australian accounting changes are made in these repositories.

File issues and PRs upstream, not here. Do not copy fork code into `ryanduguid` MIT / CC-BY-4.0 / AGPL repositories without an attribution and licence-compatibility check (see Ozzit `ATTRIBUTION.md` pattern).

Active Australian work lives in originals, notably `australian-accounting`, `accounting-review-pipeline`, `FireFalcon` (AU layer on `openfpa`), `PaciolisCube`, and the skill packs. Forks below exist only to track upstreams or to hold a contribution branch.

## Keep — active contribution

| Fork | Upstream | Why keep |
|------|----------|----------|
| `openfpa` | `JeffBrines/openfpa` | AU FP&A pack in `FireFalcon`; upstream proposal `JeffBrines/openfpa#14` |
| `openaccountants` | `openaccountants/openaccountants` | Contribution `openaccountants/openaccountants#85`; historical reference only |
| `sdk` | `meltano/sdk` | Contribution `meltano/sdk#3727` |

## Keep — tracking only, no changes

| Fork | Upstream |
|------|----------|
| `Xero-OpenAPI` | `XeroAPI/Xero-OpenAPI` |
| `xero-python` | `XeroAPI/xero-python` |
| `pyxero` | `freakboy3742/pyxero` |
| `xero-mcp-server` | `XeroAPI/xero-mcp-server` |
| `xero-command-line` | `XeroAPI/xero-command-line` |
| `requests-cache` | `requests-cache/requests-cache` |
| `django-money` | `django-money/django-money` |
| `LedgerSMB` | `ledgersmb/LedgerSMB` |
| `beancount` | `beancount/beancount` |
| `fava` | `beancount/fava` |
| `l10n-australia` | `OCA/l10n-australia` |
| `account-reconcile` | `OCA/account-reconcile` |
| `account-financial-reporting` | `OCA/account-financial-reporting` |
| `bank-statement-import` | `OCA/bank-statement-import` |
| `rest-application` | `pledger-io/rest-application` |

These are full mirrors and are large (LedgerSMB, beancount, Odoo stack). Do not mirror further to build AU packs — use a small AU layer plus an upstream link, as in `FireFalcon` / `PaciolisCube`.

Xero specs/SDKs drift fast. If a fork is more than 6 months behind with no open PR, sync it or delete it rather than letting contributors copy stale auth samples.

## Delete — off-topic

| Fork | Upstream | Reason |
|------|----------|--------|
| `gi-loadouts` | `gridhead/gi-loadouts` | Genshin Impact loadouts. Off-topic for AU-accounting profile, ~613 MB. Pending `gh repo delete ryanduguid/gi-loadouts --yes` (requires `delete_repo` scope: `gh auth refresh -h github.com -s delete_repo`). |

## Policy

1. One tracking fork per upstream. `xero-python` vs `pyxero` vs `Xero-OpenAPI` overlap — consolidate unless distinct PRs require separation. Same for `openfpa` fork vs `FireFalcon` (keep both: one pure-tracking, one AU layer).
2. Keep fork descriptions as `Fork — upstream <link>, no AU changes` where GitHub allows editing.
3. Never lodge client data, credentials, or payroll exports in forks or originals. Examples are fabricated.
