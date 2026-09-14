# install-vs.ps1 - VS BuildTools 静默安装 (修复假成功)
# 关键: 不加 --wait (会让 setup.exe 父进程假退出),改用 polling 验证 link.exe

$log = "$env:TEMP\vs-install.log"
$err = "$env:TEMP\vs-install.err"
Remove-Item $log, $err -ErrorAction SilentlyContinue

$vsSetup = 'C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe'
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] vs_BuildTools.exe exists: $(Test-Path $vsSetup)"

# --quiet 静默, --norestart 不重启, --nocache 不缓存
# --add Microsoft.VisualStudio.Workload.VCTools = "使用 C++ 的桌面开发"
# --includeRecommended 包含推荐组件 (含 Windows SDK)
# **不加 --wait** (那是假成功源)
$argList = @(
    '--quiet',
    '--norestart',
    '--nocache',
    '--add', 'Microsoft.VisualStudio.Workload.VCTools',
    '--includeRecommended'
)

Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Starting VS BuildTools install..."
$p = Start-Process -FilePath $vsSetup `
    -ArgumentList $argList `
    -RedirectStandardOutput $log `
    -RedirectStandardError $err `
    -PassThru `
    -WindowStyle Hidden

Write-Output "[$(Get-Date -Format 'HH:mm:ss')] vs_setup started (PID=$($p.Id)). Polling for MSVC link.exe..."

$msvcLinkPath = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC'
$deadline = (Get-Date).AddMinutes(20)
$pollInterval = 10 # seconds

while ((Get-Date) -lt $deadline) {
    Start-Sleep -Seconds $pollInterval
    if (Test-Path $msvcLinkPath) {
        $versions = Get-ChildItem $msvcLinkPath -ErrorAction SilentlyContinue
        foreach ($v in $versions) {
            $linkExe = Join-Path $v.FullName "bin\Hostx64\x64\link.exe"
            if (Test-Path $linkExe) {
                Write-Output "[$(Get-Date -Format 'HH:mm:ss')] MSVC installed: $($v.Name)"
                Write-Output "link.exe: $linkExe"
                Write-Output "[$(Get-Date -Format 'HH:mm:ss')] VS BuildTools install complete"
                exit 0
            }
        }
    }
    Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Still installing... (msvc dir exists: $(Test-Path $msvcLinkPath))"
}

Write-Output "[$(Get-Date -Format 'HH:mm:ss')] TIMEOUT after 20 minutes"
Write-Output "---LAST 30 LINES OF LOG---"
Get-Content $log -ErrorAction SilentlyContinue | Select-Object -Last 30
exit 1
