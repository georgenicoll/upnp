$ErrorActionPreference = "Stop"

function Ensure-CargoSubcommand {
    param(
        [Parameter(Mandatory = $true)][string]$Subcommand,
        [Parameter(Mandatory = $true)][string]$CrateName
    )

    $tool = Get-Command ("cargo-" + $Subcommand) -ErrorAction SilentlyContinue
    if (-not $tool) {
        Write-Host "==> installing $CrateName (missing cargo $Subcommand)"
        & cargo install $CrateName --locked
        if ($LASTEXITCODE -ne 0) {
            throw "failed to install $CrateName"
        }
    }
}

function Run-Step {
    param(
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string[]]$Args
    )

    Write-Host "==> $Name"
    & cargo @Args
    if ($LASTEXITCODE -ne 0) {
        throw "step failed: $Name"
    }
}

Ensure-CargoSubcommand -Subcommand "audit" -CrateName "cargo-audit"
Ensure-CargoSubcommand -Subcommand "deny" -CrateName "cargo-deny"

Run-Step -Name "fmt" -Args @("fmt", "--all", "--", "--check")
Run-Step -Name "clippy" -Args @("clippy", "--all-targets", "--all-features", "--", "-D", "warnings")
Run-Step -Name "test" -Args @("test", "--all-targets", "--all-features")
Run-Step -Name "audit" -Args @("audit")
Run-Step -Name "deny" -Args @("deny", "check")

Write-Host "all checks passed"
