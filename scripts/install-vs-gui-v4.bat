@echo off
REM install-vs-gui-v4.bat - 终极版, 用 start 让 ps1 独立运行 (关 bat 不影响安装)
REM 主人右键 "以管理员身份运行" 才能弹 UAC
REM ASCII 干净

set "ASCII_DIR=C:\TEMP\YunZaiInstall"
if not exist "%ASCII_DIR%" mkdir "%ASCII_DIR%"

set "LOG=%ASCII_DIR%\install.log"
echo [%date% %time%] Starting install-vs-gui-v4.bat > "%LOG%"

if not exist "C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe" (
    echo [%date% %time%] Downloading vs_BuildTools.exe... >> "%LOG%"
    powershell -NoProfile -Command "[Net.ServicePointManager]::SecurityProtocol=[Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri 'https://aka.ms/vs/17/release/vs_BuildTools.exe' -OutFile '%ASCII_DIR%\vs_BuildTools.exe' -UseBasicParsing" >> "%LOG%" 2>&1
) else (
    copy /Y "C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe" "%ASCII_DIR%\vs_BuildTools.exe" >> "%LOG%" 2>&1
)

copy /Y "%~dp0install-vs-elevated-v2.ps1" "%ASCII_DIR%\install-vs-elevated.ps1" >> "%LOG%" 2>&1

REM Critical: use start to detach PowerShell from this cmd
echo [%date% %time%] Starting PowerShell detached... >> "%LOG%"
start "VSBuildToolsInstaller" /B powershell -NoProfile -ExecutionPolicy Bypass -File "%ASCII_DIR%\install-vs-elevated.ps1"

echo.
echo ========================================
echo  PowerShell started in background.
echo  Even if this window closes, install continues.
echo.
echo  Monitor: notepad C:\TEMP\YunZaiInstall\install.log
echo ========================================
echo.
timeout /t 5 /nobreak >nul
exit
