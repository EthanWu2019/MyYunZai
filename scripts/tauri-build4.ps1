$env:PATH = 'C:\Users\34018\.cargo\bin;C:\Program Files\nodejs;' + $env:PATH
$env:LIB = 'C:\Program Files (x86)\Windows Kits\10\Lib\10.0.19041.0\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.19041.0\ucrt\x64'
$env:TAURI_SIGNING_PRIVATE_KEY = ''
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = ''

Set-Location E:\YunZai_APP
npm run tauri build 2>&1
