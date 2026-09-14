@echo off
REM build-yunzai-v2.bat - 主人双击打包 YunZai APP (修复一闪而过)
REM 输出全部写到日志, 末尾 pause 防止窗口消失

setlocal
set "LOG=C:\TEMP\YunZaiInstall\build.log"
set "PROJECT_DIR=E:\YunZai_APP"

if not exist "C:\TEMP\YunZaiInstall" mkdir "C:\TEMP\YunZaiInstall"
echo [%date% %time%] Starting build-yunzai-v2.bat > "%LOG%"

REM 检查 VS BuildTools 是否装好
echo [%date% %time%] Checking VS BuildTools... >> "%LOG%"
if not exist "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC" (
    echo.
    echo ========================================
    echo  ERROR: VS BuildTools not installed!
    echo  Please first run 安装VSBuildTools.bat
    echo ========================================
    echo.
    pause
    exit /b 1
)
echo [%date% %time%] VS BuildTools OK >> "%LOG%"

REM 检查 node_modules
if not exist "%PROJECT_DIR%\node_modules" (
    echo [%date% %time%] Installing npm dependencies... >> "%LOG%"
    pushd "%PROJECT_DIR%"
    call "C:\Program Files\nodejs\npm.cmd" install >> "%LOG%" 2>&1
    popd
)

REM PATH 设置
set "PATH=C:\Program Files\nodejs;C:\Users\34018\.cargo\bin;C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.40.17.10\bin\Hostx64\x64;%PATH%"
set "PATH=C:\Program Files\nodejs;C:\Users\34018\.cargo\bin;%PATH%"

cd /d "%PROJECT_DIR%"

REM cargo fetch
echo [%date% %time%] cargo fetch... >> "%LOG%"
pushd src-tauri
call "C:\Users\34018\.cargo\bin\cargo.exe" fetch >> "%LOG%" 2>&1
popd

REM tauri build
echo [%date% %time%] tauri build... >> "%LOG%"
call "C:\Program Files\nodejs\npm.cmd" run tauri build >> "%LOG%" 2>&1
set BUILD_ERR=%ERRORLEVEL%
echo [%date% %time%] tauri build exit=%BUILD_ERR% >> "%LOG%"

if exist "src-tauri\target\release\bundle\msi\*.msi" (
    echo.
    echo ========================================
    echo   BUILD SUCCESS
    echo ========================================
    explorer.exe "src-tauri\target\release\bundle\msi\"
) else (
    echo.
    echo ========================================
    echo   BUILD FAILED
    echo   Check log: %LOG%
    echo ========================================
)

echo.
echo Full log: %LOG%
pause
endlocal
