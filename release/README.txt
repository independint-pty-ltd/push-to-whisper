Push-to-Whisper v0.1.0
======================

A fast, private, and efficient push-to-speak transcription tool using OpenAI's Whisper model.

QUICK START
----------
1. Double-click push-to-whisper.exe to run
2. The application will appear in your system tray
3. Hold Right Control key to record speech
4. Release the key to transcribe at your cursor position
5. Double-press ESC to exit

FIRST RUN
---------
On first run, the application will automatically download the Whisper model file
(approximately 1.5GB). This may take a few minutes depending on your internet connection.

COMMAND LINE OPTIONS
-------------------
push-to-whisper.exe --no-beep    # Disable audio feedback
push-to-whisper.exe --no-tray    # Disable system tray
push-to-whisper.exe --no-visual  # Disable visual feedback
push-to-whisper.exe --model-size tiny.en  # Use a smaller, faster model
push-to-whisper.exe -m medium.en  # Short form for model size option
push-to-whisper.exe --force-cpu  # Force CPU mode (disable GPU acceleration)
push-to-whisper.exe --no-gpu     # Alternative syntax for forcing CPU mode

CONFIGURATION FILE
-----------------
On first run, a configuration file named "push-to-whisper.config" is created
in the same directory as the executable. You can edit this file with any text
editor to change the default settings:

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

# Save debug recording to file (true/false)
# Creates debug_recording.wav in the application directory
enable_debug_recording = false

# Force CPU mode (true/false)
# Set to true to disable GPU acceleration and use CPU only
force_cpu = false

Command line arguments will always override settings in the configuration file.

ADVANCED OPTIONS
---------------
push-to-whisper.exe --long-press-threshold 300  # Set threshold to 300ms
push-to-whisper.exe --lpt 300                   # Short form
push-to-whisper.exe --headphone-keepalive 60    # Set interval to 60s
push-to-whisper.exe --hk 0                      # Disable keepalive
push-to-whisper.exe --debug-recording           # Enable debug recording
push-to-whisper.exe --no-debug-recording        # Disable debug recording
push-to-whisper.exe --force-cpu                 # Force CPU mode
push-to-whisper.exe --no-gpu                    # Alternative for CPU mode

MODEL SIZES
-----------
The application supports different Whisper model sizes:
- tiny.en: Smallest and fastest, less accurate (approx. 75MB)
- base.en: Small and fast with decent accuracy (approx. 150MB)
- small.en: Good balance of speed and accuracy (approx. 500MB)
- medium.en: High accuracy with reasonable speed (approx. 1.5GB) - DEFAULT
- large: Highest accuracy, slowest, supports all languages (approx. 3GB)

Models with ".en" suffix are optimized for English only. The "large" model
supports all languages but requires more processing power.

SYSTEM REQUIREMENTS
------------------
- Windows 10 or Windows 11
- Microphone or audio input device
- 4GB RAM minimum (8GB recommended)
- For GPU acceleration: NVIDIA GPU with CUDA support

TROUBLESHOOTING
--------------
- No audio input: Check default microphone in Windows settings
- First-time slowness: The first transcription may be slow as the model loads
- Performance issues: Try a smaller model with --model-size tiny.en or --model-size base.en
- Text insertion problems: Make sure your cursor is in a text field
- CUDA/GPU issues: If you experience crashes or errors related to GPU acceleration, 
  use --force-cpu to run in CPU-only mode

For more information, visit: https://github.com/independint-pty-ltd/push-to-whisper