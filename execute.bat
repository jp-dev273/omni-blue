@echo off
SETLOCAL

REM --- CHECK FOR RUST INSTALLATION ---
WHERE rustc >nul 2>&1
IF ERRORLEVEL 1 (
    ECHO Rust is not installed or not found in the system PATH.
    ECHO Please install Rust from https://www.rust-lang.org/tools/install and ensure it is accessible.
    EXIT /B 1
)
ECHO Rust found. Proceeding...

REM --- CHECK FOR CARGO INSTALLATION ---
WHERE cargo >nul 2>&1
IF ERRORLEVEL 1 (
    ECHO Cargo is not installed or not found in the system PATH.
    ECHO Please install Cargo from https://www.rust-lang.org/tools/install and ensure it is accessible.
    EXIT /B 1
)
ECHO Cargo found. Proceeding...

REM --- CHECK PSEXEC FOR SYSTEM ACCESS ---
WHERE psexec >nul 2>&1
IF ERRORLEVEL 1 (
    ECHO PsExec is not installed or not found in the system PATH.
    ECHO Please download PsExec from the Sysinternals website and ensure it is accessible.
    EXIT /B 1
) 

ECHO PsExec found. Proceeding...

cargo build -p btks-win

IF ERRORLEVEL 1 (
    ECHO Build failed. Please check the Rust code for errors.
    EXIT /B 1
)

SET "PROJECT_DIR=%~dp0"
SET "EXECUTABLE_PATH=%PROJECT_DIR%target\debug\btks-win.exe"
powershell -NoProfile -Command "Start-Process cmd -Verb RunAs -ArgumentList '/k psexec -accepteula -s %EXECUTABLE_PATH%'"

ENDLOCAL