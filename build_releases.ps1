# PowerShell Script to Build and Zip Separate CUDA Releases for Push-to-Whisper

# --- Configuration ---
$ProjectName = "push-to-whisper"

# --- Dynamic Version Extraction from Git Branch ---
Write-Host "Attempting to extract version from Git branch name..."
$CurrentBranch = ""
try {
    $CurrentBranch = git rev-parse --abbrev-ref HEAD | ForEach-Object {$_.Trim()}
    if (-not $CurrentBranch) {
        throw "Git branch name is empty."
    }
    Write-Host "Current Git branch: $CurrentBranch"
}
catch {
    Write-Error "Failed to get Git branch name. Ensure 'git' is installed, in PATH, and you are in a Git repository."
    Write-Error $_.Exception.Message
    exit 1
}

$VersionPattern = "v?(\d+\.\d+\.\d+)" # Looks for vX.Y.Z or X.Y.Z, captures X.Y.Z
$Version = ""

if ($CurrentBranch -match $VersionPattern) {
    $Version = $Matches[1]
    Write-Host "Successfully extracted version '$Version' from branch '$CurrentBranch'." -ForegroundColor Green
} else {
    Write-Error "Failed to extract version from branch name '$CurrentBranch'. Expected a version pattern like 'X.Y.Z' or 'vX.Y.Z' (e.g., 'release/0.3.1' or 'release/v0.3.1')."
    exit 1
}
# --- End Dynamic Version Extraction ---

$BaseExeName = "push-to-whisper.exe"
$ReadmeSource = "README.md"
$ReadmeTarget = "README.txt"
$ReleaseDir = "release" # Final directory for zipped releases

# Define target architectures (GPU Series Name -> CUDA Compute Capability)
$Targets = @{
    "GTX900series"  = "52"  # Maxwell generation
    "RTX30series"   = "86"  # Ampere generation
    "RTX40series"   = "89"  # Ada Lovelace generation
    # Add more targets here if needed (e.g., "cpu" = "")
}

# --- Script Logic ---

# Ensure the final release directory exists
if (-not (Test-Path -Path $ReleaseDir)) {
    Write-Host "Creating release directory: $ReleaseDir"
    New-Item -ItemType Directory -Path $ReleaseDir | Out-Null
}

Write-Host "Starting Push-to-Whisper release builds for v$Version..."

foreach ($TargetGpuName in $Targets.Keys) {
    $CudaArch = $Targets[$TargetGpuName]
    $StagingDir = "target/release/staging_cuda_$CudaArch"
    $ZipFileName = "${ProjectName}-windows-x64-cuda${CudaArch}-${TargetGpuName}-v${Version}.zip"
    $FinalZipPath = Join-Path $ReleaseDir $ZipFileName

    Write-Host "`nBuilding for Target: $TargetGpuName (CUDA Arch: $CudaArch)..." -ForegroundColor Yellow

    # Set the environment variable for CUDA architecture
    $env:CUDA_ARCH = $CudaArch
    Write-Host "Set env:CUDA_ARCH=$env:CUDA_ARCH"

    # Run the release build with the cuda feature
    cargo build --release --features cuda

    # Check build success
    $ExePath = "target/release/$BaseExeName"
    if (-not (Test-Path -Path $ExePath)) {
        Write-Error "Build FAILED for CUDA Arch: $CudaArch. Executable not found at $ExePath"
        # Optional: Clear env var even on failure
        Remove-Item Env:\CUDA_ARCH -ErrorAction SilentlyContinue
        continue # Skip to the next target
    }

    Write-Host "Build successful for CUDA Arch: $CudaArch"

    # --- Staging and Zipping ---
    Write-Host "Preparing files for zipping..."

    # Clean and create staging directory
    if (Test-Path -Path $StagingDir) {
        Remove-Item -Recurse -Force $StagingDir
    }
    New-Item -ItemType Directory -Path $StagingDir | Out-Null

    # Copy executable
    Write-Host "Copying executable to staging area..." -ForegroundColor Cyan
    Copy-Item -Path $ExePath -Destination $StagingDir -Force

    # Copy and rename README
    if (Test-Path -Path $ReadmeSource) {
        Write-Host "Copying and renaming README to staging area..." -ForegroundColor Cyan
        Copy-Item -Path $ReadmeSource -Destination (Join-Path $StagingDir $ReadmeTarget) -Force
    } else {
        Write-Warning "$ReadmeSource not found. README will not be included in the zip."
    }

    # Create the zip archive using 7-Zip
    Write-Host "Creating zip file with 7z: $FinalZipPath" -ForegroundColor Cyan
    # Use 7z.exe: 'a' adds to archive, '-tzip' specifies ZIP format, '-mx=5' is default compression
    # Redirect output to Out-Null to keep the script output cleaner
    & "7z.exe" a -tzip "$FinalZipPath" "$StagingDir\*" -y | Out-Null

    # Check if 7z succeeded (basic check: zip file exists)
    if (-not (Test-Path -Path $FinalZipPath)) {
        Write-Error "7-Zip FAILED for CUDA Arch: $CudaArch. Zip file not found at $FinalZipPath"
        # Optional: Clear env var even on failure
        Remove-Item Env:\CUDA_ARCH -ErrorAction SilentlyContinue
        continue # Skip to the next target
    }

    # Clean up staging directory
    Write-Host "Cleaning up staging directory..."
    Remove-Item -Recurse -Force $StagingDir

    Write-Host "Completed packaging for $TargetGpuName (CUDA Arch: $CudaArch) -> $FinalZipPath" -ForegroundColor Green

    # Clear the environment variable for the next iteration
    Remove-Item Env:\CUDA_ARCH -ErrorAction SilentlyContinue
}

Write-Host "`nAll release builds completed and zipped to '$ReleaseDir' directory." -ForegroundColor Magenta