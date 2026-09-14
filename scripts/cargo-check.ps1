# cargo-check.ps1 - 跑 cargo check,捕获日志
$env:PATH = 'C:\Users\34018\.cargo\bin;' + $env:PATH
Set-Location E:\YunZai_APP\src-tauri
$out = cargo check 2>&1 | Out-String
$out | Out-File -Encoding UTF8 E:\YunZai_APP\cargo-check.log
Write-Output "DONE: log written, $(($out | Measure-Object -Line).Lines) lines"
