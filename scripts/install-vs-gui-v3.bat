@echo off
REM install-vs-gui-v3.bat - all-ASCII version
REM Click "Yes" on UAC, wait 5-15 min, log at C:\TEMP\YunZaiInstall\install.log

set "ASCII_DIR=C:\TEMP\YunZaiInstall"
if not exist "%ASCII_DIR%" mkdir "%ASCII_DIR%"

set "LOG=%ASCII_DIR%\install.log"
echo [%date% %time%] Starting install-vs-gui-v3.bat > "%LOG%"

REM Step 1: ensure vs_BuildTools.exe exists at ASCII path
if exist "C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe" (
    copy /Y "C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe" "%ASCII_DIR%\vs_BuildTools.exe" >> "%LOG%" 2>&1
) else (
    powershell -NoProfile -Command "[Net.ServicePointManager]::SecurityProtocol=[Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri 'https://aka.ms/vs/17/release/vs_BuildTools.exe' -OutFile '%ASCII_DIR%\vs_BuildTools.exe' -UseBasicParsing" >> "%LOG%" 2>&1
)

REM Step 2: copy ps1 to ASCII path
copy /Y "%~dp0install-vs-elevated-v2.ps1" "%ASCII_DIR%\install-vs-elevated.ps1" >> "%LOG%" 2>&1

if not exist "%ASCII_DIR%\install-vs-elevated.ps1" (
    echo [%date% %time%] ERROR: ps1 copy failed >> "%LOG%"
    echo.
    echo ERROR: install-vs-elevated.ps1 not found
    echo Looked in: %ASCII_DIR%
    echo Log: %LOG%
    pause
    exit /b 1
)

REM Step 3: mshta UAC elevation
echo [%date% %time%] Requesting UAC elevation... >> "%LOG%"
mshta vbscript:CreateObject("Shell.Application").ShellExecute("cmd.exe","/c powershell -NoProfile -ExecutionPolicy Bypass -File ""%ASCII_DIR%\install-vs-elevated.ps1"" >> ""%LOG%"" 2>&1","","runas",1)(window.close)

echo.
echo ========================================
echo  UAC prompt sent. Click Yes.
echo  If UAC did NOT appear, right-click this bat
echo  and choose "Run as administrator".
echo.
echo  Log file: %LOG%
echo  Watch progress:
echo  notepad %LOG%
echo ========================================
echo.
pause
