# sync-and-cargo-check.ps1 - 同步代码 + cargo check
Remove-Item E:\YunZai_APP -Recurse -Force -EA SilentlyContinue
New-Item E:\YunZai_APP -ItemType Directory -Force | Out-Null
Expand-Archive -Path E:\YunZai_APP.zip -DestinationPath E:\YunZai_APP\ -Force
Move-Item E:\YunZai_APP\yunzai-app\* E:\YunZai_APP\
Move-Item E:\YunZai_APP\yunzai-app\.* E:\YunZai_APP\ -Force -EA SilentlyContinue
Remove-Item E:\YunZai_APP\yunzai-app -Recurse -Force -EA SilentlyContinue
Remove-Item E:\YunZai_APP.zip
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Sync OK"
Get-ChildItem E:\YunZai_APP | Select-Object Name | Format-Table -AutoSize

$env:PATH = 'C:\Users\34018\.cargo\bin;' + $env:PATH
Set-Location E:\YunZai_APP\src-tauri
$out = cargo check 2>&1 | Out-String
$out | Out-File -Encoding UTF8 E:\YunZai_APP\cargo-check.log
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] cargo check done, $(($out | Measure-Object -Line).Lines) lines"
