@echo off
REM build.bat - ASCII-only 打包脚本 (产 .msi + nsis .exe)

setlocal
set "PATH=C:\Program Files\nodejs;C:\Users\34018\.cargo\bin;%PATH%"

cd /d "E:\YunZai_APP"

"C:\Program Files\nodejs\npm.cmd" run tauri build
endlocal
