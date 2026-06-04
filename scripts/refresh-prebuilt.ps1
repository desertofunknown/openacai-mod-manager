param(
    [string]$Version = "0.1.0",
    [switch]$Build
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
$PrebuiltDir = Join-Path $RepoRoot "prebuilt"
$LocalTestDir = Join-Path $RepoRoot "local-test"

if ($Build) {
    Push-Location $RepoRoot
    try {
        npm run tauri build
    }
    finally {
        Pop-Location
    }
}

$PortableSource = Join-Path $RepoRoot "src-tauri\target\release\OpenACAI Mod Manager.exe"
$SetupSource = Join-Path $RepoRoot "src-tauri\target\release\bundle\nsis\OpenACAI Mod Manager_${Version}_x64-setup.exe"
$MsiSource = Join-Path $RepoRoot "src-tauri\target\release\bundle\msi\OpenACAI Mod Manager_${Version}_x64_en-US.msi"

$Artifacts = @(
    @{
        Source = $PortableSource
        Name = "OpenACAI-Mod-Manager-$Version-x64-portable.exe"
    },
    @{
        Source = $SetupSource
        Name = "OpenACAI-Mod-Manager-$Version-x64-setup.exe"
    },
    @{
        Source = $MsiSource
        Name = "OpenACAI-Mod-Manager-$Version-x64.msi"
    }
)

New-Item -ItemType Directory -Force -Path $PrebuiltDir | Out-Null
New-Item -ItemType Directory -Force -Path $LocalTestDir | Out-Null

$checksumLines = New-Object System.Collections.Generic.List[string]
foreach ($artifact in $Artifacts) {
    if (-not (Test-Path -LiteralPath $artifact.Source)) {
        throw "Missing build artifact: $($artifact.Source)"
    }

    $prebuiltPath = Join-Path $PrebuiltDir $artifact.Name
    Copy-Item -LiteralPath $artifact.Source -Destination $prebuiltPath -Force

    if ($artifact.Name.EndsWith("-portable.exe", [StringComparison]::OrdinalIgnoreCase)) {
        Copy-Item -LiteralPath $artifact.Source -Destination (Join-Path $LocalTestDir $artifact.Name) -Force
    }

    $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $prebuiltPath).Hash
    $checksumLines.Add("$hash  $($artifact.Name)")
}

$checksumLines | Set-Content -Path (Join-Path $PrebuiltDir "SHA256SUMS.txt") -Encoding ASCII
Write-Host "Updated prebuilt artifacts and local test executable."
