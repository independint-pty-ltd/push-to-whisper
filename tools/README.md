# Push-to-Whisper Release Process

This document describes how to create release packages for Push-to-Whisper.

## Prerequisites

- Rust and Cargo installed
- Git repository cloned
- For Windows: PowerShell
- For macOS/Linux: `zip` command installed
- CUDA Toolkit installed (if building with GPU support)
- Code signing certificate (for signed releases)

## Creating a Release Package

The project includes scripts to automate the release packaging process. These scripts will:

1. Build the release binary with optimizations
2. Create a zip file containing:
   - The executable
   - Required DLL files
   - Configuration file template
   - README and license files
3. Name the package according to the version (e.g., `push-to-whisper-v0.1.1.zip`)

### Complete Build Process

1. First, build the main executable:
   ```
   cargo build --release --bin push-to-whisper
   ```

2. Sign the executable (Windows):
   ```powershell
   # Using Windows SDK signtool (adjust paths as needed)
   & 'C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\signtool.exe' sign /tr http://timestamp.digicert.com /td sha256 /fd sha256 /a "target\release\push-to-whisper.exe"
   ```
   Note: You'll need a code signing certificate. You can obtain one from:
   - A Certificate Authority (CA) like DigiCert, Sectigo, etc. (recommended for distribution)
   - Create a self-signed certificate (for testing only)

3. Then create the release package:
   ```
   cargo run --release --bin create_release [version]
   ```
   Replace `[version]` with the version number (e.g., `0.1.1`)

The release package will be created in `target/release/push-to-whisper-[version].zip`

### Creating a Self-Signed Certificate (Testing Only)

For testing, you can create a self-signed certificate using PowerShell:

```powershell
# Create a self-signed certificate
New-SelfSignedCertificate -Type Custom -Subject "CN=Push-to-Whisper" -KeyUsage DigitalSignature -FriendlyName "Push-to-Whisper Certificate" -CertStoreLocation "Cert:\CurrentUser\My" -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")

# Export the certificate (you'll need the thumbprint from the previous command)
$pwd = ConvertTo-SecureString -String "YourPassword" -Force -AsPlainText
Export-PfxCertificate -cert "Cert:\CurrentUser\My\<certificate-thumbprint>" -FilePath push-to-whisper.pfx -Password $pwd
```

Note: Self-signed certificates will still show a warning on other machines. For proper distribution, use a certificate from a trusted CA.

### Free Code Signing Certificate Options

There are several ways to obtain free or low-cost code signing certificates:

1. **SignPath Foundation** (Recommended for Push-to-Whisper):
   - Free, trusted certificates for open source projects
   - Process:
     1. Create an account at [SignPath.org](https://signpath.org)
     2. Link your GitHub repository
     3. Verify your open source project meets their criteria:
        - Public repository
        - Open source license (MIT in our case)
        - Active development
     4. Set up CI/CD integration (optional but recommended)
   - Benefits:
     - EV (Extended Validation) certificate
     - Fully trusted by Windows
     - No signature count limitations
     - Automatic renewal
     - Free for open source projects

2. **Other Options** (less suitable for Push-to-Whisper):
   - Microsoft Store Developer Account ($19)
   - Let's Encrypt Code Signing (not yet available)
   - DigiCert Open Source Project program
   - Academic certificates

Note: Free certificates often have limitations such as:
- Limited validity period (usually 1 year)
- Limited number of signatures
- Require regular renewal
- May still show security warnings on some systems

For Push-to-Whisper, we recommend using SignPath Foundation's certificate as it provides the best balance of trust, convenience, and cost for our open source project.

### Alternative Methods

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

### Default Version

If you don't specify a version, the scripts will default to the version in Cargo.toml.

## Release Package Contents

The release package will include:

- The executable (`push-to-whisper.exe` on Windows, `push-to-whisper` on macOS/Linux)
- Required CUDA DLLs (if built with GPU support)
- Configuration file template (`push-to-whisper.config`)
- README and license files

## Building Without CUDA Support

To build a release without CUDA support:

```
cargo build --release --no-default-features --bin push-to-whisper
cargo run --release --bin create_release [version]
```

## Creating GitHub Releases

After creating the release package:

1. Draft a new release on GitHub
2. Tag the release with the version (e.g., `v0.1.1`)
3. Write release notes describing the changes
4. Upload the zip file
5. Publish the release

## Release Checklist

Before creating a release:

- [ ] Update version number in Cargo.toml
- [ ] Update README.md with any new features or changes
- [ ] Update configuration file template if needed
- [ ] Ensure all tests pass (`cargo test`)
- [ ] Build and test with CUDA support
- [ ] Build and test without CUDA support
- [ ] Sign the executable with code signing certificate
- [ ] Create the release package
- [ ] Test the packaged application:
  - [ ] Test with GPU acceleration
  - [ ] Test with CPU-only mode
  - [ ] Test configuration file loading
  - [ ] Test all command line options
  - [ ] Verify digital signature on a clean Windows install
- [ ] Create GitHub release with release notes
- [ ] Update documentation if needed

## Troubleshooting

If the release build fails:

1. Ensure CUDA Toolkit is installed if building with GPU support
2. Clean the build directory: `cargo clean`
3. Try building the main executable first: `cargo build --release --bin push-to-whisper`
4. Check the error messages in the build output
5. If CUDA-related errors occur, try building without CUDA support using `--no-default-features`