# Push-to-Whisper v0.2.1 Release Notes

## Improvements

### CUDA Build System
- **Enhanced Architecture Reporting**: Added clear debug output to show exactly which CUDA architectures are being targeted
- **PTX Output Detection**: Added explicit detection and reporting of PTX output for future GPU compatibility
- **Architecture Format Handling**: Improved parsing of CUDA architecture specifications in various formats
- **GPU Series Identification**: Debug output now shows which GPU series each architecture corresponds to (e.g., RTX 4000, RTX 3000)

### Build Performance
- **Optimized Binary Size**: Improved handling of single architecture builds to produce smaller, optimized binaries
- **Architecture Selection**: Better enforcement of architecture selection settings to prevent unnecessary multi-architecture builds
- **Format Normalization**: Added normalization of architecture formats to handle both semicolon and comma-separated lists

## Bug Fixes
- Fixed issues with inconsistent architecture handling between environment variables and config files
- Resolved problems with mixed architecture format specifications (e.g., compute_89,sm_89 vs. 89)
- Fixed redundant architecture compilation when PTX is specified

## System Requirements
- Windows 10 or Windows 11
- 4GB RAM minimum (8GB recommended)
- For GPU acceleration: NVIDIA GPU with CUDA support (10.x, 11.x, or 12.x)

Full Changelog: v0.2.0...v0.2.1

---

Thank you for using Push to Whisper! We appreciate your feedback and suggestions for future improvements.

[Download Push to Whisper v0.2.1](https://github.com/independint-pty-ltd/push-to-whisper/releases/tag/v0.2.1) 