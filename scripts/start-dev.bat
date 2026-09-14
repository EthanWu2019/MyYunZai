@echo off
REM start-dev.bat - ASCII-only 启动器 (SSH 端不能直接跑 tauri dev 的 GUI,这个给主人双击)
REM 必须在中文项目目录的 cmd 里双击,不要用 PowerShell

setlocal
set "PATH=C:\Program Files\nodejs;C:\Users\34018\.cargo\bin;%PATH%"

REM 切到项目根
cd /d "E:\YunZai_APP"

REM 跑 tauri dev
"C:\Program Files\nodejs\npm.cmd" run tauri dev
endlocal
