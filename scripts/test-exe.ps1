# test-exe.ps1 - 启动 yunzai-app.exe 验证它能跑 + 不打扰现运行机器人
# 设计: 启动 5 秒后检查进程状态, 然后强制结束

$exePath = 'E:\YunZai_APP\src-tauri\target\release\yunzai-app.exe'
$log = 'E:\YunZai_APP\test-exe.log'

"" | Set-Content -Encoding UTF8 $log

Add-Content -Path $log -Value "=== Starting $exePath ===" -Encoding UTF8

# 先确认没在跑
$existing = Get-Process yunzai-app -EA SilentlyContinue
if ($existing) {
    Add-Content -Path $log -Value "Already running, skipping" -Encoding UTF8
    exit 1
}

# 启动
try {
    $proc = Start-Process -FilePath $exePath -PassThru -RedirectStandardOutput (Join-Path (Split-Path $log) 'test-stdout.log') -RedirectStandardError (Join-Path (Split-Path $log) 'test-stderr.log')
    Add-Content -Path $log -Value "Started PID=$($proc.Id)" -Encoding UTF8
    Start-Sleep -Seconds 5
    $p2 = Get-Process -Id $proc.Id -EA SilentlyContinue
    if ($p2) {
        Add-Content -Path $log -Value "After 5s: PID=$($p2.Id) CPU=$($p2.CPU) SessionId=$($p2.SessionId) HasExited=$($p2.HasExited)" -Encoding UTF8
        # 检查窗口
        Add-Content -Path $log -Value "MainWindowTitle='$($p2.MainWindowTitle)' MainWindowHandle=$($p2.MainWindowHandle)" -Encoding UTF8
    } else {
        Add-Content -Path $log -Value "After 5s: PROCESS EXITED" -Encoding UTF8
        Get-Content (Join-Path (Split-Path $log) 'test-stderr.log') -EA SilentlyContinue | Out-String | Out-File -Append -Encoding UTF8 $log
    }
} catch {
    Add-Content -Path $log -Value "EXCEPTION: $_" -Encoding UTF8
}

# 强制结束
Get-Process yunzai-app -EA SilentlyContinue | Stop-Process -Force
Add-Content -Path $log -Value "Killed" -Encoding UTF8

# 也清理子进程
Get-Process | Where-Object { $_.Path -like '*yunzai-app*' } | Stop-Process -Force -EA SilentlyContinue

Add-Content -Path $log -Value "=== DONE ===" -Encoding UTF8
exit 0
