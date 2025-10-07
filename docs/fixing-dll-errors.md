# Fixing CUDA DLL Errors

## The Error

If you see an error message like:
```
The code execution cannot proceed because cudart64_12.dll was not found.
Reinstalling the program may fix this problem.
```

Or similar errors mentioning:
- `cudart64_13.dll`
- `cublas64_12.dll`
- `cublasLt64_12.dll`
- Any other CUDA-related DLL

## The Solution (Simple)

**Download the latest release package from the GitHub Releases page.**

Starting with v0.4.0, all release packages include the necessary CUDA DLLs bundled with the executable. You do **NOT** need to install CUDA separately.

### Step-by-Step Fix:

1. **Go to Releases Page:**
   https://github.com/independint-pty-ltd/push-to-whisper/releases

2. **Download the appropriate build:**
   - **Universal** (recommended) - Works with RTX 30/40/50 series
   - Or choose your specific GPU series build

3. **Extract the ZIP file completely**
   - Don't run directly from the ZIP
   - Extract all files to a folder

4. **Run `push-to-whisper.exe` from the extracted folder**

5. **Ensure you have NVIDIA GPU drivers installed**
   - Update drivers: https://www.nvidia.com/download/index.aspx
   - You do NOT need the CUDA Toolkit

## Why This Happens

### If you compiled from source:
- Your build linked to specific CUDA DLLs on your system
- Those DLLs are not on other computers
- Users without CUDA Toolkit get DLL errors

### Solution for developers:
Use the release build script which bundles DLLs automatically:
```powershell
.\tools\build_releases.ps1
```

## Alternative Solutions

### Option 1: Use CPU Mode (No GPU Required)
If you just want to get the app working without GPU acceleration:

```bash
push-to-whisper.exe --force-cpu
```

Or set in config file:
```
force_cpu = true
```

This disables CUDA entirely and runs on CPU only. Slower but no DLL dependencies.

### Option 2: Install CUDA Toolkit (Not Recommended)
If you really want to install CUDA:

1. Determine which CUDA version you need:
   - If error mentions `cudart64_12.dll` → Install CUDA 12.x
   - If error mentions `cudart64_13.dll` → Install CUDA 13.x

2. Download from NVIDIA:
   https://developer.nvidia.com/cuda-downloads

3. Install the CUDA Toolkit (~3GB download)

**Note:** This is unnecessary if you use the official release packages which bundle the DLLs.

## For Developers

### Building Without DLL Issues

#### Option 1: Use the Build Script (Recommended)
```powershell
# Set your CUDA path
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6"

# Build releases with DLL bundling
.\tools\build_releases.ps1
```

This automatically:
- Detects your CUDA version
- Builds the executable
- Bundles required DLLs
- Creates release packages

#### Option 2: Build CPU-Only Version
```bash
cargo build --release --no-default-features
```

This builds without CUDA support entirely. No DLL dependencies.

#### Option 3: Manual DLL Bundling
If you built manually with `cargo build --release --features cuda`, copy these DLLs to your executable directory:

**For CUDA 12.x:**
```
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x\bin\cudart64_12.dll
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x\bin\cublas64_12.dll
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.x\bin\cublasLt64_12.dll
```

**For CUDA 13.x:**
```
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.x\bin\cudart64_13.dll
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.x\bin\cublas64_13.dll
C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v13.x\bin\cublasLt64_13.dll
```

### Verifying DLLs are Bundled

Check your release package:
```powershell
7z l release\push-to-whisper-windows-x64-cuda89-RTX40series-v0.4.0.zip
```

Should show:
```
push-to-whisper.exe
README.txt
cudart64_12.dll        (or 13)
cublas64_12.dll        (or 13)
cublasLt64_12.dll      (or 13)
CUDA_LICENSES.txt
```

## Testing on a Clean System

To verify your build works without CUDA Toolkit:

1. **Use a VM or separate computer**
2. **Only install NVIDIA GPU drivers** (not CUDA Toolkit)
3. **Extract and run your release package**
4. **Should work without DLL errors**

## Still Having Issues?

### Check GPU Drivers
```powershell
nvidia-smi
```

Should show your GPU and driver version. If this fails:
- Update GPU drivers from NVIDIA
- Ensure GPU is properly seated
- Check Windows recognizes the GPU

### Check DLL Dependencies
Use Dependency Walker or similar to see what DLLs the executable needs:
```powershell
# PowerShell command to check DLLs
dumpbin /dependents push-to-whisper.exe
```

### Enable Debug Logging
Run with verbose logging:
```bash
push-to-whisper.exe --debug-recording
```

Check the log output for CUDA initialization messages.

### Contact Support
If nothing works:
1. Try CPU mode first: `--force-cpu`
2. Report issue on GitHub with:
   - Your GPU model
   - GPU driver version (`nvidia-smi`)
   - Windows version
   - Error message
   - Whether you're using official release or built from source

## Summary

**For Users:** Download official releases from v0.4.0+ with bundled DLLs

**For Developers:** Use `.\tools\build_releases.ps1` to create proper releases

**Quick Fix:** Use `--force-cpu` flag to bypass GPU/CUDA entirely

**The bundled DLL approach means users no longer need to install the 3GB CUDA Toolkit just to run the app!**

