# final-check.ps1 - 跑完整 cargo check + vite build
$env:PATH = 'C:\Users\34018\.cargo\bin;C:\Program Files\nodejs;' + $env:PATH

Write-Output "=== cargo check ==="
Set-Location E:\YunZai_APP\src-tauri
cargo check 2>&1 | Out-File -Encoding UTF8 E:\YunZai_APP\cargo-final.log
$last = (Get-Content E:\YunZai_APP\cargo-final.log -Tail 3 -Encoding UTF8) -join " | "
Write-Output "cargo: $last"

Write-Output "=== vite build ==="
Set-Location E:\YunZai_APP
npm run build 2>&1 | Out-File -Encoding UTF8 E:\YunZai_APP\vite-final.log
$last = (Get-Content E:\YunZai_APP\vite-final.log -Tail 5 -Encoding UTF8) -join " | "
Write-Output "vite: $last"

Write-Output "=== DONE ==="
