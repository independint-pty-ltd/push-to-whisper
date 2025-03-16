# PowerShell script to create a release package for Push-to-Whisper
# This script creates a zip file containing the executable and README.txt

param (
    [string]$version = "0.1.0",
    [string]$releaseDir = "release",
    [string]$targetDir = "target/release"
)

# Ensure we're in the project root directory
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptPath
Set-Location $projectRoot

# Create output directory if it doesn't exist
if (-not (Test-Path -Path $releaseDir)) {
    New-Item -ItemType Directory -Path $releaseDir | Out-Null
    Write-Host "Created release directory: $releaseDir"
}

# Define the zip file name
$zipFileName = "push-to-whisper-v$version.zip"
$zipFilePath = Join-Path -Path $releaseDir -ChildPath $zipFileName

# Check if the executable exists
$exePath = Join-Path -Path $targetDir -ChildPath "push-to-whisper.exe"
if (-not (Test-Path -Path $exePath)) {
    Write-Host "Error: Executable not found at $exePath" -ForegroundColor Red
    Write-Host "Please build the project first with 'cargo build --release'" -ForegroundColor Yellow
    exit 1
}

# Check if README.txt exists
$readmePath = Join-Path -Path $releaseDir -ChildPath "README.txt"
if (-not (Test-Path -Path $readmePath)) {
    Write-Host "Error: README.txt not found at $readmePath" -ForegroundColor Red
    exit 1
}

# Create a temporary directory for packaging
$tempDir = Join-Path -Path $releaseDir -ChildPath "temp_package"
if (Test-Path -Path $tempDir) {
    Remove-Item -Path $tempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $tempDir | Out-Null

# Copy files to the temporary directory
Copy-Item -Path $exePath -Destination $tempDir
Copy-Item -Path $readmePath -Destination $tempDir

# Create the zip file
Write-Host "Creating release package: $zipFilePath" -ForegroundColor Green
if (Test-Path -Path $zipFilePath) {
    Remove-Item -Path $zipFilePath -Force
}

# Create the zip file
Compress-Archive -Path "$tempDir\*" -DestinationPath $zipFilePath

# Clean up the temporary directory
Remove-Item -Path $tempDir -Recurse -Force

# Check if the zip file was created successfully
if (Test-Path -Path $zipFilePath) {
    $fileSize = (Get-Item -Path $zipFilePath).Length
    $fileSizeMB = [math]::Round($fileSize / 1MB, 2)
    Write-Host "Release package created successfully: $zipFilePath ($fileSizeMB MB)" -ForegroundColor Green
    
    # List the contents of the zip file
    Write-Host "Package contents:" -ForegroundColor Cyan
    $zipEntries = (Get-ChildItem -Path $tempDir).Name
    foreach ($entry in $zipEntries) {
        Write-Host "- $entry" -ForegroundColor Cyan
    }
} else {
    Write-Host "Error: Failed to create release package" -ForegroundColor Red
}

Write-Host "`nTo create a release with a different version number:" -ForegroundColor Yellow
Write-Host ".\tools\create_release_package.ps1 -version 0.2.0" -ForegroundColor Yellow 