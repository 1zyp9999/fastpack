@echo off
REM FastPack Ultra-Fast Installer for Windows
REM Optimized for maximum speed and minimal overhead

setlocal enabledelayedexpansion

set INSTALLER_VERSION=1.0.0
set INSTALLER_NAME=FastPack Installer

set COLOR_RESET=[0m
set COLOR_GREEN=[0;32m
set COLOR_BLUE=[0;34m
set COLOR_YELLOW=[1;33m
set COLOR_RED=[0;31m

echo %COLOR_BLUE%========================================%COLOR_RESET%
echo %COLOR_BLUE%  %INSTALLER_NAME% v%INSTALLER_VERSION%%COLOR_RESET%
echo %COLOR_BLUE%========================================%COLOR_RESET%
echo.

set INSTALL_DIR=%ProgramFiles%\FastPack
set PACKAGE_NAME=FastPack
set VERSION=1.0.0

echo %COLOR_BLUE%[INFO]%COLOR_RESET% Starting installation...
echo %COLOR_BLUE%[INFO]%COLOR_RESET% Installation directory: %INSTALL_DIR%
echo.

REM Check for administrator privileges
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo %COLOR_YELLOW%[WARNING]%COLOR_RESET% Requesting administrator privileges...
    powershell -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b 0
)

REM Create installation directory
if not exist "%INSTALL_DIR%" (
    mkdir "%INSTALL_DIR%"
    echo %COLOR_GREEN%[SUCCESS]%COLOR_RESET% Created installation directory
)

REM Extract archive
set SCRIPT_PATH=%~f0
set OFFSET=0

for /f "delims=" %%A in ('findstr /n "__ARCHIVE_START__" "%SCRIPT_PATH%"') do (
    set LINE=%%A
    set /a OFFSET=!LINE!+1
    goto :found
)

:found
set /a OFFSET+=100

echo %COLOR_BLUE%[INFO]%COLOR_RESET% Extracting package archive...

powershell -Command "& { $scriptPath = '%SCRIPT_PATH%'; $offset = %OFFSET%; $content = [System.IO.File]::ReadAllBytes($scriptPath); $archive = $content[$offset..($content.Length - $offset)]; [System.IO.File]::WriteAllBytes('%TEMP%\fastpack_archive.bin', $archive); }"

if %errorLevel% neq 0 (
    echo %COLOR_RED%[ERROR]%COLOR_RESET% Failed to extract archive
    pause
    exit /b 1
)

echo %COLOR_GREEN%[SUCCESS]%COLOR_RESET% Archive extracted

REM Install files
echo %COLOR_BLUE%[INFO]%COLOR_RESET% Installing files...

powershell -Command "& { $archivePath = '%TEMP%\fastpack_archive.bin'; $installDir = '%INSTALL_DIR%'; $reader = [System.IO.BinaryReader]::new([System.IO.File]::OpenRead($archivePath)); while ($reader.BaseStream.Position -lt $reader.BaseStream.Length) { $lineLength = $reader.ReadInt32(); $line = [System.Text.Encoding]::UTF8.GetString($reader.ReadBytes($lineLength)); $parts = $line.Split('|'); if ($parts.Length -eq 2) { $path = $parts[0]; $size = [int]$parts[1]; $targetPath = Join-Path $installDir $path; $targetDir = Split-Path $targetPath; if (-not (Test-Path $targetDir)) { New-Item -ItemType Directory -Path $targetDir -Force | Out-Null }; $fileData = $reader.ReadBytes($size); [System.IO.File]::WriteAllBytes($targetPath, $fileData); } } $reader.Close(); }"

if %errorLevel% neq 0 (
    echo %COLOR_RED%[ERROR]%COLOR_RESET% Failed to install files
    pause
    exit /b 1
)

echo %COLOR_GREEN%[SUCCESS]%COLOR_RESET% Files installed

REM Clean up temporary files
del "%TEMP%\fastpack_archive.bin" >nul 2>&1

REM Add to PATH
echo %COLOR_BLUE%[INFO]%COLOR_RESET% Adding to system PATH...

setx PATH "%PATH%;%INSTALL_DIR%\bin" >nul 2>&1
if %errorLevel% equ 0 (
    echo %COLOR_GREEN%[SUCCESS]%COLOR_RESET% Added to PATH
) else (
    echo %COLOR_YELLOW%[WARNING]%COLOR_RESET% Could not add to PATH automatically
    echo %COLOR_YELLOW%[WARNING]%COLOR_RESET% Please add %INSTALL_DIR%\bin to PATH manually
)

REM Create desktop shortcut
set DESKTOP=%USERPROFILE%\Desktop
if exist "%DESKTOP%" (
    echo %COLOR_BLUE%[INFO]%COLOR_RESET% Creating desktop shortcut...
    powershell -Command "& { $WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%DESKTOP%\FastPack.lnk'); $Shortcut.TargetPath = '%INSTALL_DIR%\bin\fastpack.exe'; $Shortcut.WorkingDirectory = '%INSTALL_DIR%'; $Shortcut.Description = 'FastPack - Ultra-Fast Package Builder'; $Shortcut.Save(); }"
    echo %COLOR_GREEN%[SUCCESS]%COLOR_RESET% Desktop shortcut created
)

REM Create start menu shortcut
set START_MENU=%APPDATA%\Microsoft\Windows\Start Menu\Programs
if exist "%START_MENU%" (
    echo %COLOR_BLUE%[INFO]%COLOR_RESET% Creating start menu shortcut...
    powershell -Command "& { $WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%START_MENU%\FastPack.lnk'); $Shortcut.TargetPath = '%INSTALL_DIR%\bin\fastpack.exe'; $Shortcut.WorkingDirectory = '%INSTALL_DIR%'; $Shortcut.Description = 'FastPack - Ultra-Fast Package Builder'; $Shortcut.Save(); }"
    echo %COLOR_GREEN%[SUCCESS]%COLOR_RESET% Start menu shortcut created
)

echo.
echo %COLOR_GREEN%========================================%COLOR_RESET%
echo %COLOR_GREEN%  Installation Complete!%COLOR_RESET%
echo %COLOR_GREEN%========================================%COLOR_RESET%
echo.
echo %COLOR_BLUE%FastPack has been installed to:%COLOR_RESET%
echo   %INSTALL_DIR%
echo.
echo %COLOR_BLUE%To start FastPack:%COLOR_RESET%
echo   1. Double-click the desktop shortcut
echo   2. Or run: fastpack
echo   3. Or run: %INSTALL_DIR%\bin\fastpack.exe
echo.
echo %COLOR_YELLOW%Note:%COLOR_RESET% You may need to restart your terminal for PATH changes to take effect.
echo.

pause
exit /b 0

__ARCHIVE_START__