# Push-to-Whisper

A fast, private, and efficient push-to-speak transcription tool that uses OpenAI's Whisper model for real-time speech-to-text, running entirely on your local machine. Written in Rust for optimal performance and minimal resource usage.

> **Note:** This project was developed with the assistance of Claude 3.7 Sonnet and [Cursor](https://cursor.sh), the AI-native code editor.
>
> **Language Support:** Push-to-Whisper is optimized for English by default. While the large model supports multiple languages, the default and recommended models are English-specific for better performance.

## Features

- **Push-to-Speak**: Hold Right Control key to record speech, release to transcribe
- **Zero File Storage**: All audio processing happens in-memory
- **Cursor Position Insertion**: Transcribed text appears at your cursor location
- **Complete Privacy**: 100% local processing, no data sent to external servers
- **GPU Acceleration**: CUDA support for faster transcription when available
- **Clipboard Preservation**: Automatically saves and restores clipboard content
- **Multiple Text Insertion Methods**: Compatible with various applications
- **Audio Feedback**: Optional beep sounds for recording start/stop
- **System Tray Integration**: Easy access and status indication
- **Headphone Protection**: Prevents wireless headphones from auto-disconnecting
- **Model Selection**: Choose from different Whisper model sizes to balance accuracy and speed

## Quick Start (Windows)

1. **Download the Latest Release**:
   - Go to the [Releases](https://github.com/independint-pty-ltd/push-to-whisper/releases) page
   - Download the latest `push-to-whisper.exe` file

2. **Run the Application**:
   - Double-click the downloaded executable
   - The application will automatically download the required Whisper model on first run
   - A system tray icon will appear when the application is running

3. **Using the Tool**:
   - Hold the Right Control key to start recording
   - Speak clearly into your microphone
   - Release the key to process and insert the transcribed text at your cursor position
   - Double-press ESC to exit the application

## Command Line Options

```
push-to-whisper.exe --no-beep    # Disable audio feedback
push-to-whisper.exe --no-tray    # Disable system tray
push-to-whisper.exe --no-visual  # Disable visual feedback
push-to-whisper.exe --model-size tiny.en  # Use a smaller, faster model
push-to-whisper.exe -m medium.en  # Short form for model size option
```

## Configuration

Push-to-Whisper supports two ways to configure the application:

1. **Command Line Arguments**: As shown above, for one-time settings.
2. **Configuration File**: For persistent settings.

On first run, a `push-to-whisper.config` file is created in the same directory as the executable with default settings:

```
# Push-to-Whisper Configuration File
# Edit this file to change default settings
# Command line arguments will override these settings

# Audio feedback (true/false)
enable_beep = true

# System tray icon (true/false)
enable_tray = true

# Visual feedback (true/false)
enable_visual = true

# Whisper model size (tiny.en, base.en, small.en, medium.en, large)
model_size = medium.en

# Long press threshold in milliseconds (how long to hold the key before recording starts)
long_press_threshold = 500

# Headphone keepalive interval in seconds (prevents wireless headphones from disconnecting)
# Set to 0 to disable
headphone_keepalive_interval = 30
```

You can edit this file with any text editor to change the default behavior. Command line arguments will always override settings in the configuration file.

## Command Line Options (Advanced)

In addition to the basic options shown above, the following advanced options are available:

```
# Set the long press threshold (milliseconds)
push-to-whisper.exe --long-press-threshold 300
push-to-whisper.exe --lpt 300  # Short form

# Set the headphone keepalive interval (seconds, 0 to disable)
push-to-whisper.exe --headphone-keepalive 60
push-to-whisper.exe --hk 0  # Short form to disable

# Enable or disable debug recording (saves audio to debug_recording.wav)
push-to-whisper.exe --debug-recording     # Enable
push-to-whisper.exe --no-debug-recording  # Disable
```

## Model Sizes

The application supports different Whisper model sizes:
- **tiny.en**: Smallest and fastest, less accurate (approx. 75MB)
- **base.en**: Small and fast with decent accuracy (approx. 150MB)
- **small.en**: Good balance of speed and accuracy (approx. 500MB)
- **medium.en**: High accuracy with reasonable speed (approx. 1.5GB) - DEFAULT
- **large**: Highest accuracy, slowest, supports all languages (approx. 3GB)

Models with ".en" suffix are optimized for English only. The "large" model supports all languages but requires more processing power.

## System Requirements

- Windows 10 or Windows 11
- Microphone or audio input device
- 4GB RAM minimum (8GB recommended)
- For GPU acceleration: NVIDIA GPU with CUDA support

## Troubleshooting

- **No audio input**: Check default microphone in Windows settings
- **First-time slowness**: The first transcription may be slow as the model loads
- **Performance issues**: Try a smaller model with `--model-size tiny.en` or `--model-size base.en`
- **Text insertion problems**: Make sure your cursor is in a text field
- **Non-English languages**: Use the large model with `--model-size large` for non-English transcription

## Building from Source (For Developers)

### Prerequisites

1. Install Rust and Cargo:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Or download from [rustup.rs](https://rustup.rs/)

2. For GPU support (optional):
   - Install [CUDA Toolkit 11.7+](https://developer.nvidia.com/cuda-toolkit)

### Build Steps

1. Clone and build:
   ```
   git clone https://github.com/independint-pty-ltd/push-to-whisper
   cd push-to-whisper
   cargo build --release
   ```

2. The executable will be in `target/release/push-to-whisper.exe`

## Future Platform Support

While currently Windows-only, support for Linux and macOS is planned for future releases.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [OpenAI Whisper](https://github.com/openai/whisper) - The underlying speech recognition model
- [whisper-rs](https://github.com/tazz4843/whisper-rs) - Rust bindings for Whisper
- [cpal](https://github.com/RustAudio/cpal) - Cross-platform audio
- [rodio](https://github.com/RustAudio/rodio) - Audio playback
- [rdev](https://github.com/Narsil/rdev) - Raw device events
- [enigo](https://github.com/enigo-rs/enigo) - Keyboard control 