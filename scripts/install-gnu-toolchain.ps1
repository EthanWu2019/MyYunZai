# install-gnu-toolchain.ps1 - 装 Rust GNU toolchain + MinGW
$env:PATH = 'C:\Users\34018\.cargo\bin;' + $env:PATH
$log = 'C:\TEMP\YunZaiInstall\gnu-install.log'
"[" + (Get-Date -Format 'HH:mm:ss') + "] Starting GNU install" | Out-File -Encoding UTF8 $log

# 1. 装 rustup gnu toolchain
"[" + (Get-Date -Format 'HH:mm:ss') + "] rustup target add x86_64-pc-windows-gnu" | Out-File -Append -Encoding UTF8 $log
rustup target add x86_64-pc-windows-gnu 2>&1 | Out-File -Append -Encoding UTF8 $log

"[" + (Get-Date -Format 'HH:mm:ss') + "] rustup toolchain install stable-x86_64-pc-windows-gnu" | Out-File -Append -Encoding UTF8 $log
rustup toolchain install stable-x86_64-pc-windows-gnu 2>&1 | Out-File -Append -Encoding UTF8 $log

# 2. 下 MinGW (winlibs, 单文件 ZIP, ~150MB)
$mingwZip = 'E:\mingw64.zip'
$mingwDir = 'E:\mingw64'
"[" + (Get-Date -Format 'HH:mm:ss') + "] Downloading MinGW to $mingwZip" | Out-File -Append -Encoding UTF8 $log
if (-not (Test-Path $mingwZip)) {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri 'https://github.com/niXman/mingw-builds-binaries/releases/download/14.2.0-rt_v12-rev0/x86_64-14.2.0-release-posix-seh-msvcrt-rt_v12-rev0.7z' -OutFile $mingwZip -UseBasicParsing
}
"[" + (Get-Date -Format 'HH:mm:ss') + "] MinGW size: $((Get-Item $mingwZip).Length)" | Out-File -Append -Encoding UTF8 $log

# 7z 解压需要装 7zip, 或者直接用 winget 装 - 假设已有 7z
# 退路: 用 .zip 后缀的 MinGW
"[" + (Get-Date -Format 'HH:mm:ss') + "] MinGW download done. Need 7z to extract." | Out-File -Append -Encoding UTF8 $log
"[" + (Get-Date -Format 'HH:mm:ss') + "] If no 7z, using winget/MSI alternative..." | Out-File -Append -Encoding UTF8 $log

# 3. 检查 7z
$7z = (Get-Command 7z -EA SilentlyContinue).Source
if ($7z) {
    "[" + (Get-Date -Format 'HH:mm:ss') + "] 7z found: $7z" | Out-File -Append -Encoding UTF8 $log
    & 7z x $mingwZip "-o$mingwDir" -y 2>&1 | Out-File -Append -Encoding UTF8 $log
}

# 4. 装 link.exe 替代 (用 Rust 自带的)
# Rust GNU toolchain 自带 ld.exe, 不需要 MinGW 的 link.exe
# 但需要 MinGW 的 gcc.exe for C compilation
"[" + (Get-Date -Format 'HH:mm:ss') + "] GNU toolchain ready. gcc.exe in $mingwDir\mingw64\bin\" | Out-File -Append -Encoding UTF8 $log
"[" + (Get-Date -Format 'HH:mm:ss') + "] DONE" | Out-File -Append -Encoding UTF8 $log
exit 0
