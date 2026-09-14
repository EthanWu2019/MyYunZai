@echo off
REM install-vs-gui-v2.bat - 主人双击装 VS BuildTools (修复一闪而过)
REM 输出全部写到 C:\TEMP\YunZaiInstall\install.log, 末尾 pause 防止窗口消失

set "ASCII_DIR=C:\TEMP\YunZaiInstall"
if not exist "%ASCII_DIR%" mkdir "%ASCII_DIR%"

set "LOG=%ASCII_DIR%\install.log"
echo [%date% %time%] Starting install-vs-gui-v2.bat > "%LOG%"

REM Step 1: 检查 vs_BuildTools.exe 是否在主人的 TEMP
if exist "C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe" (
    echo [%date% %time%] Found vs_BuildTools.exe >> "%LOG%"
    copy /Y "C:\Users\34018\AppData\Local\Temp\vs_BuildTools.exe" "%ASCII_DIR%\vs_BuildTools.exe" >> "%LOG%" 2>&1
) else (
    echo [%date% %time%] ERROR: vs_BuildTools.exe NOT FOUND, downloading... >> "%LOG%"
    powershell -NoProfile -Command "[Net.ServicePointManager]::SecurityProtocol=[Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -Uri 'https://aka.ms/vs/17/release/vs_BuildTools.exe' -OutFile '%ASCII_DIR%\vs_BuildTools.exe' -UseBasicParsing" >> "%LOG%" 2>&1
)

REM Step 2: 复制 ps1 到 ASCII 路径
copy /Y "%~dp0install-vs-elevated.ps1" "%ASCII_DIR%\install-vs-elevated.ps1" >> "%LOG%" 2>&1
if not exist "%ASCII_DIR%\install-vs-elevated.ps1" (
    echo [%date% %time%] ERROR: ps1 copy failed >> "%LOG%"
    type "%LOG%"
    echo.
    echo ERROR: install-vs-elevated.ps1 not found in %ASCII_DIR%
    pause
    exit /b 1
)

REM Step 3: 通过 mshta 提权 (会弹 UAC 让主人点"是")
echo [%date% %time%] Requesting UAC elevation... >> "%LOG%"
mshta vbscript:CreateObject("Shell.Application").ShellExecute("cmd.exe","/c powershell -NoProfile -ExecutionPolicy Bypass -File ""%ASCII_DIR%\install-vs-elevated.ps1"" >> ""%LOG%"" 2>&1","","runas",1)(window.close)

echo.
echo ========================================
echo  UAC request sent. Click "Yes" if prompted.
echo  If UAC did NOT appear, right-click the bat and "Run as administrator".
echo.
echo  Log file: %LOG%
echo  Open it in Notepad to watch progress:
echo  notepad %LOG%
echo ========================================
echo.
pause
