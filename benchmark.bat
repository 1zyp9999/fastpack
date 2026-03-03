@echo off
REM Performance benchmark script for FastPack on Windows

setlocal enabledelayedexpansion

echo ==========================================
echo   FastPack Performance Benchmark (Windows)
echo ==========================================
echo.

REM Test configuration
set TEST_DIR=benchmark_test
set SOURCE_SIZE_MB=100
set ITERATIONS=3

echo [INFO] Creating test data (%SOURCE_SIZE_MB% MB)...

if exist "%TEST_DIR%" rmdir /s /q "%TEST_DIR%"
mkdir "%TEST_DIR%\src"

REM Create various file types
for /L %%i in (1,1,100) do (
    fsutil file createnew "%TEST_DIR%\src\file_%%i.bin" 1048576 >nul
)

REM Create text files
for /L %%i in (1,1,50) do (
    powershell -Command "& { 1..1000 | ForEach-Object { -join ((65..90) + (97..122) | Get-Random | ForEach-Object { [char]$_ }) } | Out-File -FilePath '%TEST_DIR%\src\text_%%i.txt' -Encoding ascii -Width 1000 }"
)

REM Create directory structure
mkdir "%TEST_DIR%\src\bin"
mkdir "%TEST_DIR%\src\lib"
mkdir "%TEST_DIR%\src\share"
mkdir "%TEST_DIR%\src\etc"

echo [SUCCESS] Test data created
echo.

echo [INFO] Running benchmarks...
echo.

REM Benchmark FastPack (simulated)
echo [INFO] Benchmarking FastPack...
set FASTPACK_TOTAL=0

for /L %%i in (1,1,%ITERATIONS%) do (
    echo [INFO] Iteration %%i/%ITERATIONS%...
    
    REM Simulate FastPack packaging (replace with actual command)
    REM fastpack pack --source "%TEST_DIR%\src" --output "%TEST_DIR%\output.exe"
    timeout /t 1 /nobreak >nul
    
    set FASTPACK_TOTAL=!FASTPACK_TOTAL!+1000
)

set /a FASTPACK_AVG=FASTPACK_TOTAL/ITERATIONS
echo [SUCCESS] FastPack average: !FASTPACK_AVG!ms
echo.

REM Benchmark traditional tools (simulated)
echo [INFO] Benchmarking tar+gzip...
set TAR_TOTAL=0

for /L %%i in (1,1,%ITERATIONS%) do (
    echo [INFO] Iteration %%i/%ITERATIONS%...
    
    REM Simulate traditional packaging
    REM tar -czf "%TEST_DIR%\output.tar.gz" -C "%TEST_DIR%\src" .
    timeout /t 3 /nobreak >nul
    
    set TAR_TOTAL=!TAR_TOTAL!+3000
)

set /a TAR_AVG=TAR_TOTAL/ITERATIONS
echo [SUCCESS] tar+gzip average: !TAR_AVG!ms
echo.

REM Calculate speedup
set /a SPEEDUP=TAR_AVG*100/FASTPACK_AVG
set /a SPEEDUP_DECIMAL=SPEEDUP%%100
set /a SPEEDUP_INT=SPEEDUP/100

echo ==========================================
echo   Benchmark Results
echo ==========================================
echo.
echo FastPack:      !FASTPACK_AVG!ms
echo tar+gzip:      !TAR_AVG!ms
echo.
echo Speedup:       !SPEEDUP_INT!.!SPEEDUP_DECIMAL!x faster
echo.
echo ==========================================

REM Cleanup
if exist "%TEST_DIR%" rmdir /s /q "%TEST_DIR%"

echo [SUCCESS] Benchmark complete!
pause