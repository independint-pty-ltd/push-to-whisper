# Building Push-to-Whisper with CUDA Support

## Quick Start

### Prerequisites
1. **Rust toolchain** (latest stable)
2. **CUDA Toolkit** 12.x or 13.x
3. **7-Zip** (for creating release packages)
4. **Git** (for version detection)

### One-Command Build
```powershell
.\tools\build_releases.ps1
```

This automatically:
- Extracts version from git branch
- Builds for multiple GPU architectures
- Bundles CUDA runtime DLLs
- Creates release packages in `release/` directory

## What Gets Built

The script builds 4 variants:

| Build Name | CUDA Arch | Target GPUs | File Name |
|------------|-----------|-------------|-----------|
| Universal | 86;89;90 | RTX 30/40/50 | `push-to-whisper-windows-x64-cudaUniversal-v*.zip` |
| RTX30series | 86 | RTX 30xx | `push-to-whisper-windows-x64-cuda86-RTX30series-v*.zip` |
| RTX40series | 89 | RTX 40xx | `push-to-whisper-windows-x64-cuda89-RTX40series-v*.zip` |
| RTX50series | 90 | RTX 50xx | `push-to-whisper-windows-x64-cuda90-RTX50series-v*.zip` |

## Environment Variables

The build script uses these environment variables:

### `CUDA_PATH` (Auto-detected)
Points to your CUDA installation. The script will:
1. Try `$env:CUDA_PATH` if set
2. Search common installation paths
3. Use first CUDA 13.x, 12.x, or 11.x found

You can set it manually:
```powershell
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6"
```

### `CUDA_ARCH` (Set by script)
Specifies which GPU architectures to build for:
```powershell
$env:CUDA_ARCH = "89"        # Single architecture (RTX 40xx)
$env:CUDA_ARCH = "86;89;90"  # Multiple architectures (Universal)
```

### `CUDA_MULTI_ARCH` (Set by script)
Enables multi-architecture builds when `CUDA_ARCH` contains semicolons:
```powershell
$env:CUDA_MULTI_ARCH = "1"
```

## What Gets Bundled

Each release package includes:

### 1. Executable
- `push-to-whisper.exe` - Main application

### 2. Documentation
- `README.txt` - User documentation (from README.md)

### 3. CUDA Runtime DLLs (Auto-bundled)

**For CUDA 12.x builds:**
- `cudart64_12.dll` - CUDA Runtime
- `cublas64_12.dll` - CUDA BLAS
- `cublasLt64_12.dll` - CUDA BLAS LightTable

**For CUDA 13.x builds:**
- `cudart64_13.dll` - CUDA Runtime
- `cublas64_13.dll` - CUDA BLAS
- `cublasLt64_13.dll` - CUDA BLAS LightTable

### 4. License
- `CUDA_LICENSES.txt` - NVIDIA CUDA redistribution license

## Build Process Details

### Step 1: Version Detection
```powershell
$CurrentBranch = git rev-parse --abbrev-ref HEAD
# Extracts version from branch like "release/v0.4.0" → "0.4.0"
```

### Step 2: Update Cargo.toml
Updates the `[package]` version to match the git branch version.

### Step 3: Build Each Target
For each architecture in `$Targets`:
```powershell
$env:CUDA_ARCH = "89"  # Example for RTX 40xx
cargo build --release --features cuda
```

### Step 4: Bundle DLLs
Script automatically:
- Detects CUDA version from `CUDA_PATH`
- Finds matching DLLs in `$CUDA_PATH\bin\`
- Copies DLLs to staging directory
- Includes CUDA license file

### Step 5: Create ZIP Package
Uses 7-Zip to create release archive:
```powershell
7z.exe a -tzip "$ReleaseDir\$ZipFileName" "$StagingDir\*"
```

## Customizing the Build

### Add New GPU Architecture
Edit `tools/build_releases.ps1`:
```powershell
$Targets = @{
    "Universal"     = "86;89;90"
    "RTX30series"   = "86"
    "RTX40series"   = "89"
    "RTX50series"   = "90"
    "GTX1000series" = "61"  # Add custom target
}
```

### Build Single Architecture Only
```powershell
# Temporarily modify $Targets to include only what you want
$Targets = @{
    "RTX40series"   = "89"
}
.\tools\build_releases.ps1
```

Or build manually:
```powershell
$env:CUDA_ARCH = "89"
cargo build --release --features cuda
```

### Skip DLL Bundling
Comment out the DLL bundling section in `build_releases.ps1` (lines ~134-169), or clear `CUDA_PATH`:
```powershell
$env:CUDA_PATH = ""
.\tools\build_releases.ps1
```

### Build CPU-Only Version
```powershell
cargo build --release --no-default-features
```

No CUDA dependencies, no DLL issues.

## Troubleshooting Builds

### Error: "CUDA_PATH not set"
**Solution:**
```powershell
# Find your CUDA installation
dir "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA"

# Set path to your version
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6"
```

### Error: "cudart64_*.dll not found"
**Cause:** CUDA Toolkit may not be properly installed.

**Solution:**
1. Reinstall CUDA Toolkit
2. Verify DLLs exist: `dir "$env:CUDA_PATH\bin\cudart*.dll"`
3. Update `CUDA_PATH` to correct version

### Error: "7z.exe not found"
**Solution:** Install 7-Zip and add to PATH:
```powershell
# Install 7-Zip
winget install 7zip.7zip

# Or download from: https://www.7-zip.org/
```

### Warning: "Building for SINGLE CUDA architecture"
This is expected when building specific GPU targets (not Universal).

To build for multiple architectures, use the Universal target or set:
```powershell
$env:CUDA_ARCH = "86;89;90"
$env:CUDA_MULTI_ARCH = "1"
cargo build --release --features cuda
```

### Binary size is too large
Multi-architecture builds are larger. If size is critical:
1. Build single-architecture versions
2. Users download their specific GPU version
3. Trade-off: More builds vs. larger files

### Build fails with CUDA 13.x
CUDA 13.x removed support for compute capability < 7.5 (Maxwell/Pascal GPUs).

**Solutions:**
- For RTX 20xx and newer: Use CUDA 13.x (works fine)
- For GTX 900/1000 series: Use CUDA 12.9 or earlier
- Update `build.rs` to target compute 7.5+ only

## Testing the Build

### 1. Verify Package Contents
```powershell
7z l release\push-to-whisper-windows-x64-cuda89-RTX40series-v0.4.0.zip
```

Should show:
```
push-to-whisper.exe
README.txt
cudart64_12.dll (or 13)
cublas64_12.dll (or 13)
cublasLt64_12.dll (or 13)
CUDA_LICENSES.txt
```

### 2. Test on Clean System
**Critical test:** Run on a system WITHOUT CUDA Toolkit installed.

1. Extract release package
2. Ensure only NVIDIA GPU drivers are installed (not CUDA)
3. Run `push-to-whisper.exe`
4. Should work without DLL errors

### 3. Test GPU Detection
```powershell
# Run the app and check logs
push-to-whisper.exe

# Should see log messages about CUDA detection
# Either: "CUDA is available, attempting to use GPU"
# Or: "CUDA is not available, using CPU mode"
```

### 4. Test CPU Fallback
```powershell
# Force CPU mode
push-to-whisper.exe --force-cpu

# Should work even without GPU
```

## Build Performance

### Compile Times (Approximate)

| Build Type | Time | Output Size |
|------------|------|-------------|
| Single arch | 10-15 min | ~25 MB (with DLLs) |
| Multi arch | 15-25 min | ~40-50 MB (with DLLs) |
| CPU only | 8-12 min | ~5 MB (no DLLs) |

Times vary based on:
- CPU performance
- Number of architectures
- Clean vs incremental build
- CUDA version

### Speeding Up Builds

**For development:**
```powershell
# Skip release optimization
cargo build --features cuda

# Or use debug mode
```

**For release:**
- Build single-architecture first (faster)
- Build multi-architecture last (slower but comprehensive)
- Use `--release` for optimizations (required for performance)

## Advanced: Build Matrix

You can build a complete matrix of releases:

```powershell
# Default: All targets
.\tools\build_releases.ps1

# Results in:
# - push-to-whisper-windows-x64-cudaUniversal-v0.4.0.zip
# - push-to-whisper-windows-x64-cuda86-RTX30series-v0.4.0.zip
# - push-to-whisper-windows-x64-cuda89-RTX40series-v0.4.0.zip
# - push-to-whisper-windows-x64-cuda90-RTX50series-v0.4.0.zip
```

Distribute all variants, letting users choose based on their GPU.

## GitHub Actions / CI

To build in CI:

```yaml
- name: Setup CUDA
  uses: Jimver/cuda-toolkit@v0.2.11
  with:
    cuda: '12.6.0'

- name: Build releases
  shell: pwsh
  run: |
    .\tools\build_releases.ps1

- name: Upload artifacts
  uses: actions/upload-artifact@v3
  with:
    name: releases
    path: release/*.zip
```

## Summary

**Simple:** `.\tools\build_releases.ps1` - Does everything automatically

**Result:** Self-contained release packages with CUDA DLLs bundled

**Users:** Download, extract, run - no CUDA installation needed

**Developers:** One script handles version, build, bundling, and packaging

For more details, see:
- `docs/cuda-compatibility.md` - Comprehensive CUDA guide
- `docs/cuda-dll-bundling-summary.md` - Implementation details
- `docs/fixing-dll-errors.md` - User troubleshooting

