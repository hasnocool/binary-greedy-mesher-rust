[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$Message,
    [switch]$All
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Ensure git repo
git rev-parse --is-inside-work-tree | Out-Null

if ($All) {
    git add -A | Out-Null
}

# Nothing staged? Try to provide a friendly message.
$staged = git diff --cached --name-only
if (-not $staged) {
    $wt = git status --porcelain
    if ($wt) {
        throw "No staged changes. Re-run with -All to stage everything, or stage files manually."
    }
    Write-Host "Nothing to commit." -ForegroundColor Yellow
    exit 0
}

git commit -m $Message

git status -sb
