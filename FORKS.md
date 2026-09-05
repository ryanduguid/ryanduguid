# Forks

Tracking-only forks. No Australian accounting changes are made in these repositories.

File issues and PRs upstream, not here. Do not copy fork code into `ryanduguid` MIT / CC-BY-4.0 / AGPL repositories without an attribution and licence-compatibility check (see Ozzit `ATTRIBUTION.md` pattern).

Active Australian work lives in originals, notably `australian-accounting`, `accounting-review-pipeline`, `au-fpa-pack` (AU layer on `openfpa`), `planning-analytics-model`, and the skill packs. Forks below exist only to track upstreams or to hold a contribution branch.

Reviewed 4 September 2026 against each fork's last commit date and its GitHub archived flag; the `l10n-australia` row was re-read on 5 September 2026 for the Odoo decision. `tools/check_links.py` checks every link here and allow-lists, by name, the forks the two archive tables record.

## Keep, active contribution

| Fork | Upstream | Why keep |
|------|----------|----------|
| [openfpa](https://github.com/ryanduguid/openfpa) | [JeffBrines/openfpa](https://github.com/JeffBrines/openfpa) | AU FP&A pack in `au-fpa-pack`; upstream proposal [JeffBrines/openfpa#14](https://github.com/JeffBrines/openfpa/pull/14) |
| [openaccountants](https://github.com/ryanduguid/openaccountants) | [openaccountants/openaccountants](https://github.com/openaccountants/openaccountants) | Contribution [openaccountants/openaccountants#85](https://github.com/openaccountants/openaccountants/pull/85); historical reference only |
| [sdk](https://github.com/ryanduguid/sdk) | [meltano/sdk](https://github.com/meltano/sdk) | Contribution [meltano/sdk#3727](https://github.com/meltano/sdk/pull/3727) |
| [xero-mcp-server](https://github.com/ryanduguid/xero-mcp-server) | [XeroAPI/xero-mcp-server](https://github.com/XeroAPI/xero-mcp-server) | Holds the bearer-token fix to `ensureError` (fork PR #1, 30 August 2026). Upstream merged the matching `formatError` fix as XeroAPI/xero-mcp-server#173 but `ensureError` still stringifies the thrown value, so no upstream PR covers it yet. The prepared upstream PR body is in [docs/xero-mcp-server-upstream-pr.md](docs/xero-mcp-server-upstream-pr.md); open it from the fork branch, then move this row to tracking only once it lands. |

## Keep, tracking only, no changes

| Fork | Upstream | Last fork commit | Six-month staleness rule |
|------|----------|------------------|--------------------------|
| [Xero-OpenAPI](https://github.com/ryanduguid/Xero-OpenAPI) | [XeroAPI/Xero-OpenAPI](https://github.com/XeroAPI/Xero-OpenAPI) | 30 August 2026 (`chore: bump version to 18.0.0`) | Flagged: Xero spec, no open PR. Current today; re-sync or delete by 28 February 2027 if nothing lands. |
| [xero-python](https://github.com/ryanduguid/xero-python) | [XeroAPI/xero-python](https://github.com/XeroAPI/xero-python) | 10 August 2026 (`PETOSS-980-fix-publish-twine-install`) | Flagged: Xero SDK, no open PR. Re-sync or delete by 10 February 2027 if nothing lands. |
| [xero-command-line](https://github.com/ryanduguid/xero-command-line) | [XeroAPI/xero-command-line](https://github.com/XeroAPI/xero-command-line) | 29 May 2026 (`chore/bump-v0.0.7`) | Flagged: Xero CLI, no open PR, three months behind on 4 September 2026. Re-sync by 29 November 2026 or delete. |

Xero specs/SDKs drift fast. If a fork is more than 6 months behind with no open PR, sync it or delete it rather than letting contributors copy stale auth samples. The three rows above carry the date on which that rule bites.

## Already archived

Read-only on GitHub as at 4 September 2026. They stay listed so nobody treats them as live tracking forks or opens a branch against them; a fork that is needed again is unarchived first.

| Fork | Upstream | Note |
|------|----------|------|
| [pyxero](https://github.com/ryanduguid/pyxero) | [freakboy3742/pyxero](https://github.com/freakboy3742/pyxero) | Overlapped `xero-python`; policy item 1 is settled by the archive. |
| [requests-cache](https://github.com/ryanduguid/requests-cache) | [requests-cache/requests-cache](https://github.com/requests-cache/requests-cache) | |
| [LedgerSMB](https://github.com/ryanduguid/LedgerSMB) | [ledgersmb/LedgerSMB](https://github.com/ledgersmb/LedgerSMB) | Large full mirror. |
| [beancount](https://github.com/ryanduguid/beancount) | [beancount/beancount](https://github.com/beancount/beancount) | Large full mirror. |
| [fava](https://github.com/ryanduguid/fava) | [beancount/fava](https://github.com/beancount/fava) | |
| [bank-statement-import](https://github.com/ryanduguid/bank-statement-import) | [OCA/bank-statement-import](https://github.com/OCA/bank-statement-import) | Odoo stack; an input to the Odoo decision below. |
| [rest-application](https://github.com/ryanduguid/rest-application) | [pledger-io/rest-application](https://github.com/pledger-io/rest-application) | |

Do not mirror further to build AU packs; use a small AU layer plus an upstream link, as in `au-fpa-pack` / `planning-analytics-model`.

## Archive

Forks with no Australian work and no reason to track. The owner archives each in the GitHub UI (Settings, Danger Zone, Archive this repository); archiving keeps the fork readable and stops contributors treating it as maintained. Nothing here archives anything.

| Fork | Upstream | Reason | When |
|------|----------|--------|------|
| [django-money](https://github.com/ryanduguid/django-money) | [django-money/django-money](https://github.com/django-money/django-money) | Upstream migrated to Codeberg; the GitHub fork tracks a repository that no longer moves. Last fork commit 15 June 2026 records the migration. | Now. |
| [l10n-australia](https://github.com/ryanduguid/l10n-australia) | [OCA/l10n-australia](https://github.com/OCA/l10n-australia) | `18.0` is the OCA scaffold (one commit, 23 July 2026). Branch `cursor/18.0-l10n-au-base-3c18` is the head of [OCA/l10n-australia#1](https://github.com/OCA/l10n-australia/pull/1), so archiving freezes that pull request. | After the Odoo decision. |
| [account-reconcile](https://github.com/ryanduguid/account-reconcile) | [OCA/account-reconcile](https://github.com/OCA/account-reconcile) | Zero AU changes; only OCA bot post-merge updates. Archived under either Odoo option. | Now. |
| [account-financial-reporting](https://github.com/ryanduguid/account-financial-reporting) | [OCA/account-financial-reporting](https://github.com/OCA/account-financial-reporting) | Zero AU changes; only OCA bot post-merge updates. Archived under either Odoo option. | Now. |

The Odoo decision (ship one real module, or archive every OCA and Odoo fork: the three rows above plus the already-archived `bank-statement-import`) is proposed in [docs/odoo-decision.md](docs/odoo-decision.md). Hold the `l10n-australia` row until it is made; the other two OCA rows and `django-money` do not depend on it.

## Delete, off-topic

| Fork | Upstream | Reason |
|------|----------|--------|
| gi-loadouts | [gridhead/gi-loadouts](https://github.com/gridhead/gi-loadouts) | Genshin Impact loadouts. Off-topic for the AU-accounting profile, ~613 MB. No longer listed under the account on 4 September 2026; if it still exists, `gh repo delete ryanduguid/gi-loadouts --yes` (requires `delete_repo` scope: `gh auth refresh -h github.com -s delete_repo`). |

## Policy

1. One tracking fork per upstream. `openfpa` fork vs `au-fpa-pack` is the deliberate exception (one pure-tracking, one AU layer).
2. Keep fork descriptions as `Fork, upstream <link>, no AU changes` where GitHub allows editing.
3. Never lodge client data, credentials, or payroll exports in forks or originals. Examples are fabricated.
4. A fork more than six months behind its upstream with no open PR is synced or deleted; a fork whose upstream has moved or whose history is empty is archived.
