# apply-topics.ps1
# Automates setting GitHub topic tags across the Australian accounting tech ecosystem repositories.
# Requires: GitHub CLI (`gh auth status`)

$ErrorActionPreference = "Stop"

$repoTopics = @{
    "ryanduguid/JohnKenley" = @("mcp-server", "model-context-protocol", "australian-taxation", "accounting", "python")
    "ryanduguid/Ozzit" = @("excel-lambda", "dynamic-arrays", "financial-modelling", "cash-flow", "tax-engine")
    "ryanduguid/CharlesHenryWickens" = @("payday-super", "superannuation", "compliance-checker", "cli", "python")
    "ryanduguid/MaryAddisonHamilton" = @("claude-code", "agent-skills", "public-practice", "australian-accounting")
    "ryanduguid/DrDebits" = @("ai-guardrails", "ethics", "apes-110", "tpb", "tax-agents")
    "ryanduguid/TheExchequerTally" = @("corporate-tax", "franking-credits", "division-203", "base-rate-entity", "python")
    "ryanduguid/SolomonsSword" = @("trust-taxation", "division-6", "section-100a", "bamford", "python")
    "ryanduguid/JohnSpenceOgilvy" = @("xero-api", "trial-balance", "data-pipeline", "pandas", "power-bi")
    "ryanduguid/hardhat-ledger" = @("claude-code", "construction-accounting", "mining-subcontractor", "australian-tax")
    "ryanduguid/ElizabethAnneAlexander" = @("zero-network", "safety-boundary", "trial-balance", "accounting-ai")
    "ryanduguid/RaymondChambers" = @("ato-benchmarks", "small-business", "variance-analysis", "python", "cli")
    "ryanduguid/RussellMathews" = @("trial-balance", "close-controls", "financial-reporting", "reconciliation", "python")
    "ryanduguid/SirAlexanderFitzgerald" = @("power-query", "vba", "excel-automation", "financial-reporting", "accounting")
    "ryanduguid/SirArthurFadden" = @("legislation-corpus", "tax-law", "australian-taxation", "python")
    "ryanduguid/tax-radar-au" = @("tax-monitor", "surveillance", "statutory-changes", "python")
    "ryanduguid/awesome-australian-accounting-tech" = @("awesome-list", "australian-accounting", "tax-tech", "open-source")
}

Write-Host "Applying repository topic tags across public repositories..." -ForegroundColor Cyan

foreach ($repo in $repoTopics.Keys) {
    $topics = $repoTopics[$repo]
    Write-Host "Updating topics for ${repo}: $($topics -join ', ')" -ForegroundColor Yellow
    $topicArgs = @($topics | ForEach-Object { "--add-topic", $_ })
    gh repo edit $repo @topicArgs
}

Write-Host "All repository topic tags successfully updated." -ForegroundColor Green
