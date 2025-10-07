# CUDA Compatibility Guide

## The CUDA DLL Problem

When you compile a CUDA application, it links against specific CUDA runtime DLLs:
- **CUDA 11.x** → `cudart64_110.dll`, `cublas64_11.dll`
- **CUDA 12.x** → `cudart64_12.dll`, `cublas64_12.dll`  
- **CUDA 13.x** → `cudart64_13.dll`, `cublas64_13.dll`

If a user doesn't have the exact CUDA version installed that your binary was compiled against, they'll get errors like:
```
The code execution cannot proceed because cudart64_12.dll was not found.
```

## Solution 1: Bundle CUDA DLLs (Current Approach)

**Status:** ✅ Implemented in `build_releases.ps1`

The build script now automatically bundles required CUDA runtime DLLs with the release package.

### What Gets Bundled:
- `cudart64_*.dll` - CUDA Runtime
- `cublas64_*.dll` - CUDA Basic Linear Algebra Subroutines
- `cublasLt64_*.dll` - CUDA BLAS LightTable

### Benefits:
- ✅ Users don't need CUDA installed
- ✅ Works out of the box
- ✅ No version conflicts
- ✅ Smaller than full CUDA installation

### Trade-offs:
- Adds ~20-50 MB to release package
- Must comply with NVIDIA redistribution terms

### NVIDIA License Compliance:
NVIDIA allows redistribution of CUDA runtime libraries. See:
https://docs.nvidia.com/cuda/eula/index.html#redistribution-rights

You are permitted to redistribute the following:
- CUDA Runtime (cudart)
- cuBLAS
- Other runtime libraries

You must:
- Include NVIDIA's license terms
- Not modify the DLLs
- Provide attribution

## Solution 2: Multi-CUDA Version Builds

**Status:** 🟡 Partially Implemented

Build separate binaries for different CUDA versions:
- `push-to-whisper-cuda11.exe` - For CUDA 11.x
- `push-to-whisper-cuda12.exe` - For CUDA 12.x
- `push-to-whisper-cuda13.exe` - For CUDA 13.x

### Benefits:
- Smaller individual downloads
- Users can choose their version

### Trade-offs:
- Confusing for users
- More builds to maintain
- Users need to know their CUDA version

## Solution 3: Dynamic CUDA Loading

**Status:** ❌ Not Implemented (Complex)

Use delay-loading or runtime loading to detect and use whatever CUDA version is available.

### Benefits:
- Single binary works with any CUDA version
- Graceful fallback to CPU

### Trade-offs:
- Requires significant code changes
- May need modifications to `whisper-rs`
- More complex error handling

## Solution 4: CPU-Only Fallback Build

**Status:** 🟡 Already Supported via Config

Build a CPU-only version as a fallback option.

```bash
cargo build --release --no-default-features
```

### Benefits:
- Works on any system
- No CUDA dependencies

### Trade-offs:
- Much slower transcription
- Still useful for testing/compatibility

## Current Build Matrix

Our current releases support:

| Build Target | CUDA Arch | GPU Series | CUDA Version | Bundled DLLs |
|-------------|-----------|------------|--------------|--------------|
| Universal   | 86;89;90  | RTX 30/40/50 | 12.x/13.x | ✅ Yes |
| RTX30series | 86        | RTX 30xx   | 12.x/13.x    | ✅ Yes |
| RTX40series | 89        | RTX 40xx   | 12.x/13.x    | ✅ Yes |
| RTX50series | 90        | RTX 50xx   | 13.x         | ✅ Yes |

## Checking Your CUDA Installation

Users can check their CUDA version:

### Windows:
```powershell
# Check if CUDA is installed
nvcc --version

# Check NVIDIA driver
nvidia-smi

# Find CUDA DLLs
dir "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\*\bin\cudart*.dll"
```

### Linux:
```bash
# Check if CUDA is installed
nvcc --version

# Check NVIDIA driver
nvidia-smi

# Find CUDA libraries
ls /usr/local/cuda*/lib64/libcudart.so*
```

## Troubleshooting

### Error: "cudart64_XX.dll not found"

**Cause:** Binary was compiled with CUDA version X.X, but user doesn't have it installed.

**Solutions:**
1. Use our bundled DLL release (recommended)
2. Install matching CUDA toolkit version
3. Use CPU-only build

### Error: "CUDA error: no CUDA-capable device detected"

**Cause:** User has CUDA installed but no compatible GPU.

**Solutions:**
1. Use CPU mode by setting `force_cpu = true` in config
2. Check GPU drivers are up to date

### Error: "CUDA error: unsupported GPU architecture"

**Cause:** GPU compute capability doesn't match the binary.

**Solutions:**
1. Use Universal build (supports multiple architectures)
2. Download binary matching GPU series

## Recommendations

### For End Users:
1. **Download the Universal build** - Works on RTX 30/40/50 series
2. DLLs are bundled - no CUDA installation needed
3. If issues persist, try CPU mode in settings

### For Developers:
1. **Build with DLL bundling** (current default)
2. Test on clean system without CUDA installed
3. Document CUDA version used for builds
4. Consider providing CPU fallback builds

## Building Locally

To build with CUDA support:

```powershell
# Set CUDA version (auto-detected from CUDA_PATH)
$env:CUDA_PATH = "C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.6"

# Build with DLL bundling
.\tools\build_releases.ps1
```

The script will automatically:
1. Detect CUDA version from path
2. Build with appropriate architecture flags
3. Bundle matching CUDA runtime DLLs
4. Create self-contained release packages

## Future Considerations

### Static Linking
Some CUDA libraries can be statically linked, but this:
- Significantly increases binary size
- May not be supported by all libraries
- Requires special linker flags

### CUDA Forward Compatibility
NVIDIA provides forward compatibility where older CUDA runtimes can work with newer drivers. This could allow:
- Building with CUDA 11.x
- Running on systems with CUDA 12.x/13.x drivers
- Broader compatibility with smaller binaries

However, you lose access to newer CUDA features and optimizations.

