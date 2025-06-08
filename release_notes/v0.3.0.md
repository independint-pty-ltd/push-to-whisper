# Push-to-Whisper v0.3.0 Release Notes

## New Features
- **Build Process**: The release build script (`build_releases.ps1`) now dynamically extracts the version number from the current Git branch name, streamlining the release packaging process.

## Improvements
- **Recording Start**: Recording now commences immediately when the hotkey is pressed, synchronized with the start of the auditory beep, ensuring no initial speech is missed.
- **Reduced Latency**: Default long-press threshold for initiating recording has been significantly reduced to ~150ms for a more responsive, near-instant feel.
- **Audio Feedback**: Default beep volume has been adjusted for a more balanced user experience.
- **Build Script Robustness**: The build script's version extraction logic is now more flexible, supporting branch names with or without a 'v' prefix for the version (e.g., `release/0.3.0` and `release/v0.3.0`).

## Bug Fixes
- No specific user-facing bug fixes in this version, primarily focused on improvements and build process enhancements.

## Performance
- No specific performance changes in this version.

## Documentation
- No documentation changes in this version.

## System Requirements
- Windows 10 or Windows 11
- 4GB RAM minimum (8GB recommended)
- For GPU acceleration: NVIDIA GPU with CUDA support (10.x, 11.x, or 12.x)

## Coming in Future Releases
- [Planned feature]
- [Planned feature]

Full Changelog: v0.2.1...v0.3.0

---

Thank you for using Push to Whisper! We appreciate your feedback and suggestions for future improvements.

[Download Push to Whisper v0.3.0](https://github.com/independint-pty-ltd/push-to-whisper/releases/tag/v0.3.0) 