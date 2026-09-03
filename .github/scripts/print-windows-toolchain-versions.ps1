$ErrorActionPreference = "Stop"

$vswhere = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\vswhere.exe"
$vsPath = (& $vswhere -latest -products * -property installationPath).Trim()
$vsVersion = (& $vswhere -latest -products * -property installationVersion).Trim()
if (-not $vsPath -or -not $vsVersion) {
    throw "Could not detect the Visual Studio installation"
}

$crtVersionFile = Join-Path $vsPath "VC\Auxiliary\Build\Microsoft.VCToolsVersion.default.txt"
$crtVersion = (Get-Content $crtVersionFile -Raw).Trim()

$sdkIncludePath = Join-Path ${env:ProgramFiles(x86)} "Windows Kits\10\Include"
$sdkVersion = Get-ChildItem $sdkIncludePath -Directory |
    Where-Object Name -Match '^\d+\.\d+\.\d+\.\d+$' |
    Sort-Object { [version]$_.Name } -Descending |
    Select-Object -First 1 -ExpandProperty Name
if (-not $sdkVersion) {
    throw "Could not detect an installed Windows SDK"
}

Write-Output "XWIN_VERSION=$($vsVersion.Split('.')[0])"
Write-Output "XWIN_SDK_VERSION=$sdkVersion"
Write-Output "XWIN_CRT_VERSION=$crtVersion"
