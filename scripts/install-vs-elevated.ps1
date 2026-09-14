# install-vs-elevated.ps1 - 以管理员身份运行的实际安装脚本
# 由 install-vs-gui.bat 提权启动

$log = "$env:TEMP\vs-install.log"
$err = "$env:TEMP\vs-install.err"
Remove-Item $log, $err -ErrorAction SilentlyContinue

$vsSetup = 'C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe'
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Running as admin: $(([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))"
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] vs_BuildTools.exe exists: $(Test-Path $vsSetup)"

$argList = @(
    '--quiet',
    '--norestart',
    '--nocache',
    '--add', 'Microsoft.VisualStudio.Workload.VCTools',
    '--includeRecommended'
)

# 这次不要 --wait, 让 vs_setup 异步跑,我们 polling 验证
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Starting vs_setup async..."
$p = Start-Process -FilePath $vsSetup `
    -ArgumentList $argList `
    -RedirectStandardOutput $log `
    -RedirectStandardError $err `
    -PassThru `
    -WindowStyle Normal

Write-Output "[$(Get-Date -Format 'HH:mm:ss')] vs_setup started (PID=$($p.Id))"
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Polling for MSVC link.exe..."

$msvcLinkPath = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC'
$deadline = (Get-Date).AddMinutes(20)

while ((Get-Date) -lt $deadline) {
    Start-Sleep -Seconds 15
    if (Test-Path $msvcLinkPath) {
        $versions = Get-ChildItem $msvcLinkPath -ErrorAction SilentlyContinue
        foreach ($v in $versions) {
            $linkExe = Join-Path $v.FullName "bin\Hostx64\x64\link.exe"
            if (Test-Path $linkExe) {
                Write-Output "[$(Get-Date -Format 'HH:mm:ss')] DONE - MSVC $($v.Name) installed"
                Write-Output "link.exe: $linkExe"
                Read-Host "Press Enter to close"
                exit 0
            }
        }
    }
    Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Still installing..."
}

Write-Output "[$(Get-Date -Format 'HH:mm:ss')] TIMEOUT after 20 minutes"
Read-Host "Press Enter to close"
exit 1
