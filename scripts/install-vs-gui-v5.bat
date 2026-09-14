@echo off
REM install-vs-gui-v5.bat - UAC-aware version
REM If UAC is OFF, we MUST run as admin via right-click (no prompt will appear)
REM If UAC is ON, mshta runas will pop a prompt
REM Strategy: try both ways

set "ASCII_DIR=C:\TEMP\YunZaiInstall"
if not exist "%ASCII_DIR%" mkdir "%ASCII_DIR%"
set "LOG=%ASCII_DIR%\install.log"
echo [%date% %time%] Starting install-vs-gui-v5.bat > "%LOG%"

REM Detect: is UAC enabled?
set "UAC_VALUE="
for /f "tokens=2*" %%a in ('reg query "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" /v ConsentPromptBehaviorAdmin 2^>nul') do set "UAC_VALUE=%%b"
echo [%date% %time%] UAC ConsentPromptBehaviorAdmin=%UAC_VALUE% >> "%LOG%"

if not exist "%ASCII_DIR%\vs_BuildTools.exe" (
    if exist "C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe" (
        copy /Y "C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe" "%ASCII_DIR%\vs_BuildTools.exe" >> "%LOG%" 2>&1
    ) else (
        echo [%date% %time%] Downloading vs_BuildTools.exe... >> "%LOG%"
        powershell -NoProfile -Command "[Net.ServicePointManager]::SecurityProtocol=[Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri 'https://aka.ms/vs/17/release/vs_BuildTools.exe' -OutFile '%ASCII_DIR%\vs_BuildTools.exe' -UseBasicParsing" >> "%LOG%" 2>&1
    )
)

copy /Y "%~dp0install-vs-elevated-v2.ps1" "%ASCII_DIR%\install-vs-elevated.ps1" >> "%LOG%" 2>&1

REM Run install-vs-elevated.ps1 directly in current cmd session
REM (since UAC is off, no prompt needed; if UAC on, will fail silently)
echo [%date% %time%] Running installer (no elevation needed since UAC may be off)... >> "%LOG%"
start "VSBuildToolsInstaller" /MIN powershell -NoProfile -ExecutionPolicy Bypass -File "%ASCII_DIR%\install-vs-elevated.ps1"

echo.
echo ========================================
echo  Installer started in background window.
echo  A separate PowerShell window may have opened.
echo  If you see a UAC prompt, click Yes.
echo.
echo  Monitor progress:
echo  notepad C:\TEMP\YunZaiInstall\install.log
echo.
echo  If install log shows only "Requesting UAC elevation..."
echo  and nothing happens, you need to:
echo    Right-click this bat -^> "Run as administrator"
echo ========================================
echo.
timeout /t 8 /nobreak >nul
exit
