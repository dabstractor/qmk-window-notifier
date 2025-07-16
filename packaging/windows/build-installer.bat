@echo off
setlocal enabledelayedexpansion

echo QMK Window Notifier Installer Build Script
echo ==========================================

REM Check if WiX is installed
where candle.exe >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: WiX Toolset not found!
    echo Please install WiX Toolset v3.x from https://wixtoolset.org/releases/
    echo Or install via package manager:
    echo   Chocolatey: choco install wixtoolset
    echo   Winget: winget install WiXToolset.WiX
    exit /b 1
)

where light.exe >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: WiX Toolset not found!
    echo Please install WiX Toolset v3.x from https://wixtoolset.org/releases/
    exit /b 1
)

echo WiX Toolset found

REM Build Rust application
echo Building Rust application...
pushd ..\..

echo Cleaning previous build...
cargo clean

echo Building in release mode...
cargo build --release
if %errorlevel% neq 0 (
    echo ERROR: Cargo build failed
    popd
    exit /b 1
)

REM Verify executable exists
if not exist "target\release\qmk-window-notifier.exe" (
    echo ERROR: Executable not found
    popd
    exit /b 1
)

echo Rust application built successfully
popd

REM Build installer
echo Building Windows installer...

echo Compiling WiX source...
candle.exe installer.wxs
if %errorlevel% neq 0 (
    echo ERROR: WiX compilation failed
    exit /b 1
)

echo Creating MSI package...
light.exe installer.wixobj -ext WixUIExtension -ext WixUtilExtension -out "QMK-Window-Notifier-Setup.msi"
if %errorlevel% neq 0 (
    echo ERROR: MSI creation failed
    exit /b 1
)

REM Clean up intermediate files
del installer.wixobj >nul 2>&1
del *.wixpdb >nul 2>&1

echo.
echo SUCCESS: Installer created successfully: QMK-Window-Notifier-Setup.msi
echo.
echo Installation Instructions:
echo 1. Run QMK-Window-Notifier-Setup.msi as Administrator
echo 2. The service will be installed and started automatically
echo 3. System tray icon should appear after installation
echo.
echo Service Management:
echo - View service: services.msc (look for 'QMK Window Notifier')
echo - Manual control: sc start/stop QMKWindowNotifier
echo - Logs: Windows Event Viewer ^> Applications and Services Logs

endlocal