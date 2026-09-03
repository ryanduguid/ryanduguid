# Forks

Tracking-only forks. No Australian accounting changes are made in these repositories.

File issues and PRs upstream, not here. Do not copy fork code into `ryanduguid` MIT / CC-BY-4.0 / AGPL repositories without an attribution and licence-compatibility check (see Ozzit `ATTRIBUTION.md` pattern).

Active Australian work lives in originals, notably `australian-accounting`, `accounting-review-pipeline`, `FireFalcon` (AU layer on `openfpa`), `PaciolisCube`, and the skill packs. Forks below exist only to track upstreams or to hold a contribution branch.

## Keep — active contribution

| Fork | Upstream | Why keep |
|------|----------|----------|
| [openfpa](https://github.com/ryanduguid/openfpa) | [JeffBrines/openfpa](https://github.com/JeffBrines/openfpa) | AU FP&A pack in `FireFalcon`; upstream proposal [JeffBrines/openfpa#14](https://github.com/JeffBrines/openfpa/pull/14) |
| [openaccountants](https://github.com/ryanduguid/openaccountants) | [openaccountants/openaccountants](https://github.com/openaccountants/openaccountants) | Contribution [openaccountants/openaccountants#85](https://github.com/openaccountants/openaccountants/pull/85); historical reference only |
| [sdk](https://github.com/ryanduguid/sdk) | [meltano/sdk](https://github.com/meltano/sdk) | Contribution [meltano/sdk#3727](https://github.com/meltano/sdk/pull/3727) |

## Keep — tracking only, no changes

| Fork | Upstream |
|------|----------|
| [Xero-OpenAPI](https://github.com/ryanduguid/Xero-OpenAPI) | [XeroAPI/Xero-OpenAPI](https://github.com/XeroAPI/Xero-OpenAPI) |
| [xero-python](https://github.com/ryanduguid/xero-python) | [XeroAPI/xero-python](https://github.com/XeroAPI/xero-python) |
| [pyxero](https://github.com/ryanduguid/pyxero) | [freakboy3742/pyxero](https://github.com/freakboy3742/pyxero) |
| [xero-mcp-server](https://github.com/ryanduguid/xero-mcp-server) | [XeroAPI/xero-mcp-server](https://github.com/XeroAPI/xero-mcp-server) |
| [xero-command-line](https://github.com/ryanduguid/xero-command-line) | [XeroAPI/xero-command-line](https://github.com/XeroAPI/xero-command-line) |
| [requests-cache](https://github.com/ryanduguid/requests-cache) | [requests-cache/requests-cache](https://github.com/requests-cache/requests-cache) |
| [django-money](https://github.com/ryanduguid/django-money) | [django-money/django-money](https://github.com/django-money/django-money) |
| [LedgerSMB](https://github.com/ryanduguid/LedgerSMB) | [ledgersmb/LedgerSMB](https://github.com/ledgersmb/LedgerSMB) |
| [beancount](https://github.com/ryanduguid/beancount) | [beancount/beancount](https://github.com/beancount/beancount) |
| [fava](https://github.com/ryanduguid/fava) | [beancount/fava](https://github.com/beancount/fava) |
| [l10n-australia](https://github.com/ryanduguid/l10n-australia) | [OCA/l10n-australia](https://github.com/OCA/l10n-australia) |
| [account-reconcile](https://github.com/ryanduguid/account-reconcile) | [OCA/account-reconcile](https://github.com/OCA/account-reconcile) |
| [account-financial-reporting](https://github.com/ryanduguid/account-financial-reporting) | [OCA/account-financial-reporting](https://github.com/OCA/account-financial-reporting) |
| [bank-statement-import](https://github.com/ryanduguid/bank-statement-import) | [OCA/bank-statement-import](https://github.com/OCA/bank-statement-import) |
| [rest-application](https://github.com/ryanduguid/rest-application) | [pledger-io/rest-application](https://github.com/pledger-io/rest-application) |

These are full mirrors and are large (LedgerSMB, beancount, Odoo stack). Do not mirror further to build AU packs — use a small AU layer plus an upstream link, as in `FireFalcon` / `PaciolisCube`.

Xero specs/SDKs drift fast. If a fork is more than 6 months behind with no open PR, sync it or delete it rather than letting contributors copy stale auth samples.

## Delete — off-topic

| Fork | Upstream | Reason |
|------|----------|--------|
| [gi-loadouts](https://github.com/ryanduguid/gi-loadouts) | [gridhead/gi-loadouts](https://github.com/gridhead/gi-loadouts) | Genshin Impact loadouts. Off-topic for AU-accounting profile, ~613 MB. Pending `gh repo delete ryanduguid/gi-loadouts --yes` (requires `delete_repo` scope: `gh auth refresh -h github.com -s delete_repo`). |

## Policy

1. One tracking fork per upstream. `xero-python` vs `pyxero` vs `Xero-OpenAPI` overlap — consolidate unless distinct PRs require separation. Same for `openfpa` fork vs `FireFalcon` (keep both: one pure-tracking, one AU layer).
2. Keep fork descriptions as `Fork — upstream <link>, no AU changes` where GitHub allows editing.
3. Never lodge client data, credentials, or payroll exports in forks or originals. Examples are fabricated.
