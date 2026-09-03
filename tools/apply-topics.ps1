# apply-topics.ps1
# Automates setting GitHub topic tags across the Australian accounting tech ecosystem repositories.
# Requires: GitHub CLI (`gh auth status`)

$ErrorActionPreference = "Stop"

$repoTopics = @{
    "ryanduguid/australian-accounting" = @("mcp-server", "model-context-protocol", "australian-taxation", "accounting", "python", "australian-accounting")
    "ryanduguid/Ozzit" = @("excel-lambda", "dynamic-arrays", "financial-modelling", "cash-flow", "gst")
    "ryanduguid/payday-super-checker" = @("payday-super", "superannuation", "sg-charge", "cli", "python")
    "ryanduguid/australian-accounting-skills" = @("claude-code", "agent-skills", "public-practice", "australian-accounting")
    "ryanduguid/DrDebits" = @("ai-guardrails", "ethics", "apes-110", "tpb", "tax-agents")
    "ryanduguid/TheExchequerTally" = @("corporate-tax", "franking-credits", "division-203", "base-rate-entity", "python")
    "ryanduguid/SolomonsSword" = @("trust-taxation", "division-6", "section-100a", "bamford", "python")
    "ryanduguid/xero-trial-balance-export" = @("xero-api", "trial-balance", "data-pipeline", "pandas", "power-bi")
    "ryanduguid/hardhat-ledger" = @("claude-code", "construction-accounting", "mining-subcontractor", "australian-tax")
    "ryanduguid/xero-ledger-review-gate" = @("zero-network", "safety-boundary", "trial-balance", "accounting-ai", "xero", "ledger-review")
    "ryanduguid/au-tax-legislation-corpus" = @("legislation-corpus", "tax-law", "australian-taxation", "python")
    "ryanduguid/awesome-australian-accounting-tech" = @("awesome-list", "australian-accounting", "tax-tech", "open-source")
    "ryanduguid/ato-benchmark-compare" = @("ato-benchmarks", "small-business", "variance-analysis", "python", "cli")
    "ryanduguid/accounting-review-pipeline" = @("trial-balance", "close-controls", "financial-reporting", "reconciliation", "python", "monthly-close")
    "ryanduguid/workpaper-review-gate" = @("accounting", "accounting-controls", "australia", "bas", "cli", "month-end", "public-practice", "python", "quality-control", "review", "review-workflow", "workpaper-review", "workpapers", "year-end")
    "ryanduguid/australian-accounting-power-bi" = @("accounting", "accounting-analytics", "australia", "australian-accounting", "dax", "financial-analytics", "payday-super", "pbip", "powerbi", "tmdl")
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
