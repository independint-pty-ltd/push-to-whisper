@echo off
REM Simple batch file wrapper for the create_release Rust script

REM Default version if not provided
set VERSION=%1
if "%VERSION%"=="" set VERSION=0.1.0

REM Build the release binary if needed
if not exist "target\release\push-to-whisper.exe" (
    echo Building release binary...
    cargo build --release
)

REM Build and run the create_release script
echo Creating release package for version %VERSION%...
cargo run --release --bin create_release %VERSION% 