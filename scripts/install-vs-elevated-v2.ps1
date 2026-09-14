# install-vs-elevated-v2.ps1 - all-ASCII, no Chinese in code paths
# Called by install-vs-gui-v3.bat via mshta runas

$asciiDir = 'C:\TEMP\YunZaiInstall'
$log = Join-Path $asciiDir 'install.log'
$vsSetup = Join-Path $asciiDir 'vs_BuildTools.exe'

function Log($msg) {
    $ts = Get-Date -Format 'HH:mm:ss'
    $line = "[$ts] $msg"
    Write-Output $line
    Add-Content -Path $log -Value $line -Encoding UTF8
}

try {
    $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    Log "=== install-vs-elevated-v2 STARTED (admin=$isAdmin) ==="

    if (-not (Test-Path $vsSetup)) {
        Log "ERROR: vs_BuildTools.exe missing at $vsSetup"
        Read-Host "Press Enter to close"
        exit 1
    }

    Log "vs_BuildTools.exe size: $((Get-Item $vsSetup).Length) bytes"

    Log "Starting vs_setup.exe async (no --wait)..."
    $p = Start-Process -FilePath $vsSetup `
        -ArgumentList @('--quiet','--norestart','--nocache','--add','Microsoft.VisualStudio.Workload.VCTools','--includeRecommended') `
        -RedirectStandardOutput (Join-Path $asciiDir 'vs-setup.log') `
        -RedirectStandardError (Join-Path $asciiDir 'vs-setup.err') `
        -PassThru `
        -WindowStyle Normal

    Log "vs_setup started PID=$($p.Id), polling for link.exe..."

    $msvcRoot = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC'
    $deadline = (Get-Date).AddMinutes(25)

    while ((Get-Date) -lt $deadline) {
        Start-Sleep -Seconds 20
        if (Test-Path $msvcRoot) {
            $vers = Get-ChildItem $msvcRoot -ErrorAction SilentlyContinue
            foreach ($v in $vers) {
                $le = Join-Path $msvcRoot "$v\bin\Hostx64\x64\link.exe"
                if (Test-Path $le) {
                    Log "DONE MSVC=$v link.exe=$le"
                    Read-Host "Press Enter to close"
                    exit 0
                }
            }
        }
        Log "Still installing... msvc_dir_exists=$(Test-Path $msvcRoot)"
    }

    Log "TIMEOUT after 25 minutes"
    Read-Host "Press Enter to close"
    exit 1
} catch {
    Log "EXCEPTION: $_"
    Read-Host "Press Enter to close"
    exit 1
}
