# install-rust.ps1 - Rust 静默安装 (放后台,主人不感知)
# 调用: powershell -NoProfile -ExecutionPolicy Bypass -File install-rust.ps1

$log = "$env:TEMP\rust-install.log"
$err = "$env:TEMP\rust-install.err"
Remove-Item $log, $err -ErrorAction SilentlyContinue

Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Starting Rust install..."
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] rustup-init.exe exists: $(Test-Path "$env:TEMP\rustup-init.exe")"

# -y 接受默认 (-q 静默 --default-toolchain stable --default-host x86_64-pc-windows-msvc --profile minimal --no-modify-path)
$argList = @(
    '-y',
    '--default-toolchain', 'stable',
    '--default-host', 'x86_64-pc-windows-msvc',
    '--profile', 'minimal',
    '--no-modify-path'
)

$p = Start-Process -FilePath "$env:TEMP\rustup-init.exe" `
    -ArgumentList $argList `
    -RedirectStandardOutput $log `
    -RedirectStandardError $err `
    -PassThru `
    -WindowStyle Hidden

$p.WaitForExit()
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] rustup-init exit code: $($p.ExitCode)"

if ($p.ExitCode -ne 0) {
    Write-Output "---STDOUT---"
    Get-Content $log -ErrorAction SilentlyContinue
    Write-Output "---STDERR---"
    Get-Content $err -ErrorAction SilentlyContinue
    exit $p.ExitCode
}

# 把 cargo bin 加到当前用户 PATH (写注册表)
$userPath = [System.Environment]::GetEnvironmentVariable('PATH', 'User')
if ($userPath -notlike '*\.cargo\bin*') {
    [System.Environment]::SetEnvironmentVariable('PATH', "$userPath;C:\Users\$env:USERNAME\.cargo\bin", 'User')
    Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Added cargo to HKCU PATH"
}

# 验证
$env:PATH += ';C:\Users\' + $env:USERNAME + '\.cargo\bin'
$rustcOut = & "$env:USERPROFILE\.cargo\bin\rustc.exe" --version 2>&1
$cargoOut = & "$env:USERPROFILE\.cargo\bin\cargo.exe" --version 2>&1
Write-Output "rustc: $rustcOut"
Write-Output "cargo: $cargoOut"
Write-Output "[$(Get-Date -Format 'HH:mm:ss')] Rust install complete"
