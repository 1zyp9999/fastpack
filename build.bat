@echo off
REM FastPack Build Script for Windows

setlocal enabledelayedexpansion

echo ==========================================
echo   FastPack Build Script (Windows)
echo ==========================================
echo.

REM Check prerequisites
echo [INFO] Checking prerequisites...

set MISSING=

where node >nul 2>&1
if %errorlevel% neq 0 set MISSING=%MISSING% node

where npm >nul 2>&1
if %errorlevel% neq 0 set MISSING=%MISSING% npm

where cargo >nul 2>&1
if %errorlevel% neq 0 set MISSING=%MISSING% cargo

if defined MISSING (
    echo [ERROR] Missing prerequisites: %MISSING%
    echo.
    echo Please install the following:
    echo   - Node.js: https://nodejs.org/
    echo   - Rust: https://www.rust-lang.org/tools/install
    exit /b 1
)

echo [SUCCESS] All prerequisites found
echo.

REM Parse command line arguments
set COMMAND=%1
if "%COMMAND%"=="" set COMMAND=build

if "%COMMAND%"=="build" goto :build
if "%COMMAND%"=="dev" goto :dev
if "%COMMAND%"=="clean" goto :clean
if "%COMMAND%"=="release" goto :release

echo Usage: %~nx0 [build^|dev^|clean^|release]
echo.
echo Commands:
echo   build    - Build the application for release
echo   dev      - Run in development mode
echo   clean    - Remove build artifacts
echo   release  - Build and create installer
exit /b 1

:build
echo [INFO] Installing dependencies...
call npm install
if %errorlevel% neq 0 (
    echo [ERROR] Failed to install dependencies
    exit /b 1
)

echo [SUCCESS] Dependencies installed
echo.

echo [INFO] Building FastPack...
call npm run build
if %errorlevel% neq 0 (
    echo [ERROR] Build failed
    exit /b 1
)

echo [SUCCESS] Build complete!
echo.
echo Binary location: src-tauri\target\release\fastpack.exe
goto :end

:dev
echo [INFO] Installing dependencies...
call npm install
if %errorlevel% neq 0 (
    echo [ERROR] Failed to install dependencies
    exit /b 1
)

echo [INFO] Starting development mode...
call npm run dev
goto :end

:clean
echo [INFO] Cleaning build artifacts...
if exist src-tauri\target rmdir /s /q src-tauri\target
if exist node_modules rmdir /s /q node_modules
if exist package-lock.json del /f /q package-lock.json
echo [SUCCESS] Clean complete!
goto :end

:release
echo [INFO] Building release version...
call :build
if %errorlevel% neq 0 exit /b 1

echo.
echo [INFO] Creating installer...
echo [INFO] Installer will be in src-tauri\target\release\bundle\
echo.
echo [SUCCESS] Release build complete!
goto :end

:end
endlocal