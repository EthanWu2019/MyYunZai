$env:PATH = 'C:\Users\34018\.cargo\bin;' + $env:PATH
$env:LIB = 'C:\Program Files (x86)\Windows Kits\10\Lib\10.0.19041.0\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.19041.0\ucrt\x64'

Set-Location E:\YunZai_APP\src-tauri
cargo check 2>&1 | Out-File -Encoding UTF8 E:\YunZai_APP\cargo-check5.log
$lines = (Get-Content E:\YunZai_APP\cargo-check5.log -Encoding UTF8 | Measure-Object -Line).Lines
Write-Output "DONE $lines lines"
