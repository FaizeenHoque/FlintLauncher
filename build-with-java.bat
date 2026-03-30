@echo off
REM Build Flint Launcher with bundled Java
REM This script bundles Java files into the installer (optional but recommended)

echo.
echo ==========================================
echo Flint Launcher - Build with Java Bundle
echo ==========================================
echo.
echo This will download and bundle Java (1-2GB)
echo Build time: 15-30 minutes
echo.
pause

echo Bundling Java files...
call node src-tauri\bundle-java.js

if errorlevel 1 (
    echo Java bundling failed, but continuing with build...
) else (
    echo Java bundled successfully!
)

echo.
echo Building Tauri application...
call npm run tauri build

echo.
echo ==========================================
echo Build complete!
echo ==========================================
echo.
pause
