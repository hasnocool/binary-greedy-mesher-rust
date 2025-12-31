[CmdletBinding(SupportsShouldProcess = $true, ConfirmImpact = 'High')]
param(
    [ValidateSet('soft','mixed','hard')][string]$Mode = 'mixed'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Ensure git repo
git rev-parse --is-inside-work-tree | Out-Null

# Ensure there is a previous commit to go back to
$hasParent = $true
try {
    git rev-parse --verify HEAD~1 | Out-Null
} catch {
    $hasParent = $false
}

if (-not $hasParent) {
    throw "No previous commit to roll back to (need at least 2 commits)."
}

# Warn if working tree is dirty
$dirty = git status --porcelain
if ($dirty -and $Mode -eq 'hard') {
    Write-Warning "Working tree has uncommitted changes. 'hard' will discard them."
}

$target = 'HEAD~1'
if ($PSCmdlet.ShouldProcess("reset --$Mode $target", "Rollback last commit")) {
    git reset --$Mode $target
    git status -sb
}
