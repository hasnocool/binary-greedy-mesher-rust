[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$RepoName,
    [string]$RemoteName = 'origin',
    [switch]$Private,
    [switch]$ForceSetRemote,
    [string]$DotEnvPath = (Join-Path (Get-Location) '.env')
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot '_github.ps1')

# Ensure git repo
git rev-parse --is-inside-work-tree | Out-Null

$branch = (git branch --show-current).Trim()
if (-not $branch) {
    throw "Could not determine current branch."
}

# Ensure at least one commit
try {
    git rev-parse HEAD | Out-Null
} catch {
    throw "No commits yet. Commit first, then run this script."
}

$token = Get-GitHubTokenFromDotEnv -DotEnvPath $DotEnvPath
$headers = New-GitHubAuthHeaders -Token $token
$owner = Get-GitHubLogin -Headers $headers

# Create repo (or reuse if it exists)
$body = @{ name = $RepoName; private = [bool]$Private } | ConvertTo-Json
try {
    $repo = Invoke-RestMethod -Method Post -Headers $headers -Uri 'https://api.github.com/user/repos' -Body $body
} catch {
    $status = $_.Exception.Response.StatusCode.Value__
    if ($status -eq 422) {
        $repo = Invoke-RestMethod -Method Get -Headers $headers -Uri "https://api.github.com/repos/$owner/$RepoName"
    } else {
        throw
    }
}

$remoteUrl = $repo.clone_url

# Set remote
$hasRemote = $false
try {
    git remote get-url $RemoteName | Out-Null
    $hasRemote = $true
} catch { }

if ($hasRemote -and -not $ForceSetRemote) {
    $existing = (git remote get-url $RemoteName).Trim()
    if ($existing -ne $remoteUrl) {
        throw "Remote '$RemoteName' already exists and differs. Re-run with -ForceSetRemote to overwrite. Existing: $existing"
    }
} else {
    if ($hasRemote) {
        git remote remove $RemoteName | Out-Null
    }
    git remote add $RemoteName $remoteUrl | Out-Null
}

# Push using Basic auth header derived from .env token
$basic = Get-GitBasicAuthHeaderValue -Login $owner -Token $token

git -c "http.extraHeader=AUTHORIZATION: $basic" push -u $RemoteName $branch

git remote -v
