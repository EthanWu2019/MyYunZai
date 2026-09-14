# tauri-build.ps1 - 在 Windows 端跑 tauri build
$env:PATH = 'C:\Users\34018\.cargo\bin;C:\Program Files\nodejs;' + $env:PATH
$env:LIB = 'C:\Program Files (x86)\Windows Kits\10\Lib\10.0.19041.0\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.19041.0\ucrt\x64'

$logPath = 'E:\YunZai_APP\tauri-build.log'
"" | Set-Content $logPath

Set-Location E:\YunZai_APP

# 前端 build
"=== vite build ===" | Out-File -Append -Encoding UTF8 $logPath
npm run build 2>&1 | Out-File -Append -Encoding UTF8 $logPath

# tauri build
"=== tauri build ===" | Out-File -Append -Encoding UTF8 $logPath
npm run tauri build 2>&1 | Out-File -Append -Encoding UTF8 $logPath

# 检查产物
"=== checking artifacts ===" | Out-File -Append -Encoding UTF8 $logPath
$msi = Get-ChildItem 'E:\YunZai_APP\src-tauri\target\release\bundle\msi\*.msi' -EA SilentlyContinue | Select-Object -First 1
$nsis = Get-ChildItem 'E:\YunZai_APP\src-tauri\target\release\bundle\nsis\*.exe' -EA SilentlyContinue | Select-Object -First 1
if ($msi) { "MSI: $($msi.FullName) size=$($msi.Length)" | Out-File -Append -Encoding UTF8 $logPath }
if ($nsis) { "NSIS: $($nsis.FullName) size=$($nsis.Length)" | Out-File -Append -Encoding UTF8 $logPath }

"=== DONE at $(Get-Date -Format 'HH:mm:ss') ===" | Out-File -Append -Encoding UTF8 $logPath
exit 0
