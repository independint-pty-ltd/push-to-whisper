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

# --- Update Cargo.toml [package] version only ---
Write-Host "Updating Cargo.toml [package] version to '$Version'..."
$CargoTomlPath = "Cargo.toml"
if (-not (Test-Path -Path $CargoTomlPath)) { Write-Error "Cargo.toml not found at $CargoTomlPath"; exit 1 }

try {
    $lines = Get-Content -Path $CargoTomlPath -Encoding UTF8
    $inPackage = $false
    $result = @()
    foreach ($line in $lines) {
        if ($line -match '^\s*\[package\]\s*$') { $inPackage = $true }
        elseif ($inPackage -and $line -match '^\s*\[') { $inPackage = $false }

        if ($inPackage -and $line -match '^\s*version\s*=\s*"[^"]+"') {
            $result += ('version = "' + $Version + '"')
        } else {
            $result += $line
        }
    }
    Set-Content -Path $CargoTomlPath -Value $result -Encoding UTF8
    Write-Host "Updated [package].version to $Version" -ForegroundColor Green
} catch {
    Write-Error "Failed to update Cargo.toml [package] version: $($_.Exception.Message)"; exit 1
}
# --- End update ---

$BaseExeName = "push-to-whisper.exe"
$ReadmeSource = "README.md"
$ReadmeTarget = "README.txt"
$ReleaseDir = "release" # Final directory for zipped releases

# Define target architectures (GPU Series Name -> CUDA Compute Capability)
$Targets = @{
    "Universal"     = "86;89;90"  # Multi-architecture build (works on all supported GPUs)
    "RTX30series"   = "86"  # Ampere generation
    "RTX40series"   = "89"  # Ada Lovelace generation
    "RTX50series"   = "90"  # Blackwell/RTX 50 generation
}

# Note: CUDA 13 removes offline compilation support for compute < 7.5.
# We skip packaging for Maxwell/Pascal when building with CUDA 13 toolchain.

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

    # Check if this is a multi-architecture build
    if ($CudaArch -match ";") {
        $env:CUDA_MULTI_ARCH = "1"
        Write-Host "Set env:CUDA_MULTI_ARCH=1 (Multi-architecture build enabled)"
    } else {
        # Make sure it's not set for single-arch builds
        Remove-Item Env:\CUDA_MULTI_ARCH -ErrorAction SilentlyContinue
    }

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

    # Copy CUDA runtime DLLs if CUDA_PATH is set
    if ($env:CUDA_PATH) {
        Write-Host "Looking for CUDA runtime DLLs to bundle..." -ForegroundColor Cyan
        $CudaBinPath = Join-Path $env:CUDA_PATH "bin"
        
        # Determine which DLL version to look for based on CUDA path
        $CudaDlls = @()
        if ($env:CUDA_PATH -match "v13\.") {
            $CudaDlls = @("cudart64_13.dll", "cublas64_13.dll", "cublasLt64_13.dll")
        } elseif ($env:CUDA_PATH -match "v12\.") {
            $CudaDlls = @("cudart64_12.dll", "cublas64_12.dll", "cublasLt64_12.dll")
        } elseif ($env:CUDA_PATH -match "v11\.") {
            $CudaDlls = @("cudart64_110.dll", "cublas64_11.dll", "cublasLt64_11.dll")
        }
        
        $CopiedDlls = 0
        foreach ($dll in $CudaDlls) {
            $dllPath = Join-Path $CudaBinPath $dll
            if (Test-Path -Path $dllPath) {
                Copy-Item -Path $dllPath -Destination $StagingDir -Force
                Write-Host "  Bundled: $dll" -ForegroundColor Green
                $CopiedDlls++
            } else {
                Write-Warning "  CUDA DLL not found: $dll (checked: $dllPath)"
            }
        }
        
        if ($CopiedDlls -eq 0) {
            Write-Warning "No CUDA DLLs were bundled. Users will need CUDA installed to run this binary."
        } else {
            Write-Host "Successfully bundled $CopiedDlls CUDA runtime DLL(s)" -ForegroundColor Green
        }
    } else {
        Write-Warning "CUDA_PATH not set. CUDA DLLs will not be bundled."
        Write-Warning "Users will need CUDA installed to run this binary."
    }

    # Copy and rename README
    if (Test-Path -Path $ReadmeSource) {
        Write-Host "Copying and renaming README to staging area..." -ForegroundColor Cyan
        Copy-Item -Path $ReadmeSource -Destination (Join-Path $StagingDir $ReadmeTarget) -Force
    } else {
        Write-Warning "$ReadmeSource not found. README will not be included in the zip."
    }

    # Copy CUDA license file if DLLs were bundled
    if ($CopiedDlls -gt 0) {
        $CudaLicensePath = "CUDA_LICENSES.txt"
        if (Test-Path -Path $CudaLicensePath) {
            Write-Host "Copying CUDA license file..." -ForegroundColor Cyan
            Copy-Item -Path $CudaLicensePath -Destination $StagingDir -Force
        }
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

    # Clear the environment variables for the next iteration
    Remove-Item Env:\CUDA_ARCH -ErrorAction SilentlyContinue
    Remove-Item Env:\CUDA_MULTI_ARCH -ErrorAction SilentlyContinue
}

Write-Host "`nAll release builds completed and zipped to '$ReleaseDir' directory." -ForegroundColor Magenta