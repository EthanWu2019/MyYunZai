@echo off
REM install-vs-gui.bat - 主人双击以管理员身份装 VS BuildTools
REM 这是 SSH 静默装失败的 fallback (Windows service session 限制)
REM 实际安装时间 5-15 分钟,期间游戏本照常用

setlocal
cd /d "%~dp0"

REM 用 PowerShell 提权 (会弹 UAC 让主人点"是")
powershell -NoProfile -Command "Start-Process -FilePath '%~dp0install-vs-elevated.ps1' -Verb RunAs"
endlocal
