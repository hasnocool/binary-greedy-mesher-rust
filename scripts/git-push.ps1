[CmdletBinding()]
param(
    [string]$Remote = 'origin',
    [string]$Branch,
    [string]$DotEnvPath = (Join-Path (Get-Location) '.env')
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot '_github.ps1')

# Ensure git repo
git rev-parse --is-inside-work-tree | Out-Null

if (-not $Branch -or $Branch.Trim().Length -eq 0) {
    $Branch = (git branch --show-current).Trim()
}
if (-not $Branch) {
    throw "Could not determine branch to push. Pass -Branch explicitly."
}

$token = Get-GitHubTokenFromDotEnv -DotEnvPath $DotEnvPath
$headers = New-GitHubAuthHeaders -Token $token
$owner = Get-GitHubLogin -Headers $headers

# Push using Basic auth header derived from .env token
$basic = Get-GitBasicAuthHeaderValue -Login $owner -Token $token

git -c "http.extraHeader=AUTHORIZATION: $basic" push -u $Remote $Branch

git status -sb
