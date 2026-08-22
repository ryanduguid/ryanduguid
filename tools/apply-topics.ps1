# apply-topics.ps1
# Automates setting GitHub topic tags across the Australian accounting tech ecosystem repositories.
# Requires: GitHub CLI (`gh auth status`)

$ErrorActionPreference = "Stop"

$repoTopics = @{
    "ryanduguid/au-tax-mcp-server" = @("mcp-server", "model-context-protocol", "australian-taxation", "accounting", "python")
    "ryanduguid/Ozzit" = @("excel-lambda", "dynamic-arrays", "financial-modelling", "cash-flow", "gst")
    "ryanduguid/payday-super-checker" = @("payday-super", "superannuation", "sg-charge", "cli", "python")
    "ryanduguid/australian-accounting-skills" = @("claude-code", "agent-skills", "public-practice", "australian-accounting")
    "ryanduguid/DrDebits" = @("ai-guardrails", "ethics", "apes-110", "tpb", "tax-agents")
    "ryanduguid/TheExchequerTally" = @("corporate-tax", "franking-credits", "division-203", "base-rate-entity", "python")
    "ryanduguid/SolomonsSword" = @("trust-taxation", "division-6", "section-100a", "bamford", "python")
    "ryanduguid/xero-trial-balance-export" = @("xero-api", "trial-balance", "data-pipeline", "pandas", "power-bi")
    "ryanduguid/hardhat-ledger" = @("claude-code", "construction-accounting", "mining-subcontractor", "australian-tax")
    "ryanduguid/xero-ai-review-gateway" = @("zero-network", "safety-boundary", "trial-balance", "accounting-ai")
    "ryanduguid/au-tax-legislation-corpus" = @("legislation-corpus", "tax-law", "australian-taxation", "python")
    "ryanduguid/tax-radar-au" = @("synthetic-demo", "review-queue", "statutory-changes", "python")
    "ryanduguid/awesome-australian-accounting-tech" = @("awesome-list", "australian-accounting", "tax-tech", "open-source")
    "ryanduguid/ato-benchmark-compare" = @("ato-benchmarks", "small-business", "variance-analysis", "python", "cli")
    "ryanduguid/monthly-close-control-plane" = @("trial-balance", "close-controls", "financial-reporting", "reconciliation", "python")
    "ryanduguid/accounting-excel-toolkit" = @("power-query", "vba", "excel-automation", "financial-reporting", "accounting")
}

Write-Host "Applying repository topic tags across public repositories..." -ForegroundColor Cyan

foreach ($repo in $repoTopics.Keys) {
    $topics = $repoTopics[$repo]
    Write-Host "Updating topics for ${repo}: $($topics -join ', ')" -ForegroundColor Yellow
    $topicArgs = @($topics | ForEach-Object { "--add-topic", $_ })
    gh repo edit $repo @topicArgs
}

Write-Host "All repository topic tags successfully updated." -ForegroundColor Green
