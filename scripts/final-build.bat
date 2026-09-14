@echo off
REM final-build.bat - 主人装好 VS BuildTools 后,双击这个打 .msi
REM 前置: 主人桌面"安装VSBuildTools.bat" 已装过 + node_modules 已 npm install 过
REM 跑完会出: E:\YunZai_APP\src-tauri\target\release\bundle\msi\*.msi
REM          E:\YunZai_APP\src-tauri\target\release\bundle\nsis\*.exe

setlocal
set "PATH=C:\Program Files\nodejs;C:\Users\34018\.cargo\bin;%PATH%"
cd /d "E:\YunZai_APP"

REM 1. 装前端依赖 (如果还没装)
if not exist "node_modules\" (
    echo Installing npm dependencies...
    "C:\Program Files\nodejs\npm.cmd" install
)

REM 2. cargo fetch (拉所有 Rust crate 一次)
cd src-tauri
"C:\Users\34018\.cargo\bin\cargo.exe" fetch

REM 3. release build + 打包
cd ..
"C:\Program Files\nodejs\npm.cmd" run tauri build

if exist "src-tauri\target\release\bundle\msi\*.msi" (
    echo.
    echo ========================================
    echo   BUILD SUCCESS
    echo ========================================
    explorer.exe "src-tauri\target\release\bundle\msi\"
    explorer.exe "src-tauri\target\release\bundle\nsis\"
) else (
    echo.
    echo BUILD FAILED - check output above
)
endlocal
pause
