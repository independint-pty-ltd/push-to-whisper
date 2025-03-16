# Push-to-Whisper Release Process

This document describes how to create release packages for Push-to-Whisper.

## Prerequisites

- Rust and Cargo installed
- Git repository cloned
- For Windows: PowerShell
- For macOS/Linux: `zip` command installed

## Creating a Release Package

The project includes scripts to automate the release packaging process. These scripts will:

1. Build the release binary if needed
2. Create a zip file containing the executable and README.txt
3. Name the package according to the version (e.g., `push-to-whisper-v0.1.1.zip`)

### Using the Scripts

#### On Windows

```
.\tools\create_release.bat [version]
```

Example:
```
.\tools\create_release.bat 0.1.1
```

#### On macOS/Linux

```
chmod +x ./tools/create_release.sh  # Make executable (first time only)
./tools/create_release.sh [version]
```

Example:
```
./tools/create_release.sh 0.1.1
```

#### Using Cargo Directly (All Platforms)

```
cargo run --release --bin create_release [version]
```

Example:
```
cargo run --release --bin create_release 0.1.1
```

### Default Version

If you don't specify a version, the scripts will default to `0.1.0`.

## Release Package Contents

The release package will include:

- The executable (`push-to-whisper.exe` on Windows, `push-to-whisper` on macOS/Linux)
- README.txt with usage instructions

## Manual Release Process

If you prefer to create the release package manually:

1. Build the release binary:
   ```
   cargo build --release
   ```

2. Copy the executable from `target/release/` and the README.txt from `release/` to a new directory

3. Create a zip file containing these files

## Building Without CUDA Support

To build a release without CUDA support:

```
cargo build --release --no-default-features
```

Then create the release package as usual.

## Creating GitHub Releases

After creating the release package:

1. Draft a new release on GitHub
2. Tag the release with the version (e.g., `v0.1.1`)
3. Write release notes describing the changes
4. Upload the zip file
5. Publish the release

## Release Checklist

- [ ] Update version number in Cargo.toml
- [ ] Update README.md with any new features or changes
- [ ] Update README.txt in the release directory
- [ ] Ensure all tests pass
- [ ] Build and test the release binary
- [ ] Create the release package
- [ ] Test the packaged application
- [ ] Create GitHub release with release notes
- [ ] Update documentation if needed 