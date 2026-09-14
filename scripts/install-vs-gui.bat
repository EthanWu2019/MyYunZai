@echo off
REM install-vs-gui.bat - 主人双击装 VS BuildTools
REM 这个 bat 必须 ASCII, 不包含中文
REM 原理: 复制 ps1 到 ASCII 路径, 然后通过 cmd /c 提权调用

REM 1. 把 ps1 复制到 ASCII 临时目录
set "ASCII_DIR=C:\TEMP\YunZaiInstall"
if not exist "%ASCII_DIR%" mkdir "%ASCII_DIR%"
copy /Y "%~dp0install-vs-elevated.ps1" "%ASCII_DIR%\install-vs-elevated.ps1" >nul

REM 2. 通过 mshta 弹 UAC 提权 (经典 cmd 提权方法, 不嵌套 PowerShell 变量)
mshta vbscript:CreateObject("Shell.Application").ShellExecute("cmd.exe","/c powershell -NoProfile -ExecutionPolicy Bypass -File ""%ASCII_DIR%\install-vs-elevated.ps1""","","runas",1)(window.close)
