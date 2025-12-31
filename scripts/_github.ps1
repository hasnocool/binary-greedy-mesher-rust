Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Get-DotEnvValue {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string[]]$Keys
    )

    if (-not (Test-Path -LiteralPath $Path)) {
        throw "Missing .env file at: $Path"
    }

    $lines = Get-Content -LiteralPath $Path
    foreach ($key in $Keys) {
        foreach ($line in $lines) {
            $trim = $line.Trim()
            if ($trim.Length -eq 0) { continue }
            if ($trim.StartsWith('#')) { continue }

            # Match KEY=VALUE (value can contain '=')
            if ($trim -match ('^\s*' + [regex]::Escape($key) + '\s*=\s*(.+)\s*$')) {
                $value = $Matches[1].Trim()
                $value = $value.Trim('"').Trim("'")
                if ($value.Length -eq 0) { break }
                return $value
            }
        }
    }

    throw "No GitHub token found in .env. Tried keys: $($Keys -join ', ')"
}

function Get-GitHubTokenFromDotEnv {
    param(
        [string]$DotEnvPath = (Join-Path (Get-Location) '.env')
    )

    return Get-DotEnvValue -Path $DotEnvPath -Keys @(
        'GITHUB_API_KEY',
        'GITHUB_TOKEN',
        'GITHUB_CLASSIC_TOKEN'
    )
}

function New-GitHubAuthHeaders {
    param(
        [Parameter(Mandatory = $true)][string]$Token
    )

    return @{
        Authorization = "token $Token"
        'User-Agent'  = 'greedy-rust-mesh-scripts'
        Accept        = 'application/vnd.github+json'
    }
}

function Get-GitHubLogin {
    param(
        [Parameter(Mandatory = $true)][hashtable]$Headers
    )

    return (Invoke-RestMethod -Headers $Headers -Uri 'https://api.github.com/user').login
}

function Get-GitBasicAuthHeaderValue {
    param(
        [Parameter(Mandatory = $true)][string]$Login,
        [Parameter(Mandatory = $true)][string]$Token
    )

    $pair = "$Login`:$Token"
    $b64 = [Convert]::ToBase64String([Text.Encoding]::ASCII.GetBytes($pair))
    return "basic $b64"
}
