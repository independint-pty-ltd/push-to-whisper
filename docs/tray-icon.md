# 🎯 Push-to-Whisper System Tray Icon Guide

## Overview

Push-to-Whisper now features a comprehensive system tray icon that provides visual feedback and easy access to configuration settings. The tray icon changes color based on the application state and offers a convenient menu for managing the application. Additionally, the app now shows visual foreground notifications during recording and transcribing for enhanced user awareness.

## 🎨 Visual States

### Icon Design
- **Shape**: Circular icons with anti-aliased edges for a modern, professional look
- **Size**: 16x16 pixels optimized for system tray display
- **Style**: Smooth circular design with transparent background

### Icon Colors
- **🔘 Grey Circle** - Normal/Ready state
- **🔴 Red Circle** - Recording in progress
- **🟠 Amber/Orange Circle** - Transcribing/Processing

### Tooltips
- **"Push-to-Whisper - Ready"** - Application is idle and ready
- **"Push-to-Whisper - Recording..."** - Currently recording audio
- **"Push-to-Whisper - Transcribing..."** - Processing recorded audio

## 🖼️ Visual Foreground Notifications

### Notification Window Features
- **Always-on-Top**: Small notification window appears in center of screen
- **Transparent Background**: Modern, non-intrusive design
- **Animated Indicators**: Animated dots show activity progress
- **Timer Display**: Shows elapsed time for current operation
- **Auto-Close**: Automatically disappears when returning to normal state

### Notification States
- **🔴 RECORDING**: Red indicator with "Listening..." animated text
- **🟠 TRANSCRIBING**: Amber indicator with "Processing..." animated text
- **Auto-Hide**: No notification shown for normal state

### Notification Behavior
- **Non-Blocking**: Notifications don't interfere with other applications
- **Thread-Safe**: Runs in separate thread for smooth performance
- **State-Aware**: Automatically updates when state changes
- **Performance Optimized**: Minimal CPU and memory usage

## 🖱️ Interaction

### Left Click
- **Action**: Opens the Settings window directly
- **Purpose**: Quick access to configuration

### Right Click
- **Action**: Shows context menu with options:
  - **Settings** - Opens the configuration window
  - **About** - Shows application information
  - **Exit** - Closes the application

## ⚙️ Settings Window Features

### 🔧 General Settings
- **🔊 Enable audio feedback** - Toggle beep sounds
- **📍 Enable system tray icon** - Show/hide tray icon (requires restart)
- **👁️ Enable visual feedback** - Toggle visual notifications and foreground windows
- **🔊 Beep volume** - Adjust audio feedback volume (0.0 - 1.0)

### 🎵 Audio Settings
- **⏱️ Long press threshold** - How long to hold key before recording starts (10-2000ms)
  - *Default: 50ms (optimized for v0.3.1)*
  - *Description: How long to hold the key before recording starts*
- **🎧 Headphone keepalive interval** - Prevents wireless disconnection (0-120s)
  - *Description: Prevents wireless headphones from disconnecting (0 = disabled)*
- **🐛 Enable debug recording** - Save audio for troubleshooting
  - *Description: Saves audio to debug_recording.wav for troubleshooting*

### 🤖 Whisper AI Settings
- **📦 Model size** - Choose AI model for transcription:
  - **Tiny (~75MB)** - Fastest, least accurate
  - **Base (~150MB)** - Fast, decent accuracy
  - **Small (~500MB)** - Good balance
  - **Medium (~1.5GB)** - High accuracy (Default)
  - **Large (~3GB)** - Highest accuracy, all languages
- **💻 Force CPU mode** - Disable GPU acceleration
  - *Description: Use this if you have GPU issues or want to save power*

## 🔄 Configuration Management

### Saving Settings
1. Make changes in the settings window
2. Click **💾 Save** to apply changes
3. Settings are automatically written to `push-to-whisper.config`

### Visual Feedback Control
- **Enable/Disable**: Toggle visual feedback in General Settings
- **Immediate Effect**: Changes apply without restart
- **Performance Impact**: Minimal when enabled, zero when disabled

### Restart Requirements
Some settings require an application restart:
- **System tray icon** enable/disable
- **Model size** changes
- **Force CPU mode** changes

When restart is required:
- ⚠️ Warning message appears in settings window
- **🔄 Restart Now** button becomes available
- Click to automatically restart the application

### Reset to Defaults
- Click **🔄 Reset to Defaults** to restore original settings
- This will require a restart to take effect

## 🛠️ Technical Implementation

### Circular Icon Generation
- **Anti-Aliased Edges**: Smooth circular appearance with gradient transparency
- **Optimized Rendering**: Fast RGBA buffer generation for each state
- **Memory Efficient**: Minimal memory footprint for icon data
- **Cross-Platform**: Compatible with Windows system tray requirements

### Visual Notification System
- **Dedicated Thread**: Notifications run in separate thread for performance
- **Non-Blocking**: Main application remains responsive during notifications
- **State Synchronization**: Real-time updates when application state changes
- **Resource Management**: Automatic cleanup when notifications close

### Thread Safety
- **Tray Icon Thread**: Isolated tray icon operations for stability
- **Notification Thread**: Separate thread for foreground notifications
- **Thread-Safe Communication**: Message channels for cross-thread updates
- **Performance Optimized**: Minimal overhead for thread coordination

### Error Handling
- **Graceful Fallbacks**: Robust error recovery for UI operations
- **Notification Failures**: App continues working if notifications fail
- **Resource Cleanup**: Proper disposal of UI resources on errors
- **Comprehensive Logging**: Detailed error information for troubleshooting

## 🔧 Configuration File

Settings are stored in `push-to-whisper.config`:

```ini
# Audio feedback settings
disable_beep = false
beep_volume = 0.1

# System tray settings  
disable_tray = false
disable_visual = false

# Audio processing settings
long_press_threshold = 50
headphone_keepalive_interval = 0
enable_debug_recording = false

# Whisper AI settings
model_size = "medium.en"
force_cpu = false
```

## 🚀 Quick Start

1. **Launch Application** - Circular tray icon appears grey (ready state)
2. **Watch Visual Feedback** - Icon changes color and notifications appear during operation
3. **Left-click** tray icon to open settings
4. **Configure** visual feedback preferences
5. **Save** settings and enjoy enhanced visual experience

## 🎯 Best Practices

### Visual Experience
- **Keep Visual Feedback Enabled** - Provides clear indication of app status
- **Monitor Notifications** - Watch for recording/transcribing indicators
- **Circular Icon Benefits** - Modern appearance integrates well with Windows 11

### Performance
- **Minimal Impact** - Visual features designed for efficiency
- **Disable if Needed** - Can turn off visual feedback to save resources
- **Thread Isolation** - UI operations don't affect audio processing

### User Experience
- **Clear State Indication** - Always know what the app is doing
- **Non-Intrusive Design** - Notifications don't block other work
- **Quick Access** - Settings always one click away

## 🔍 Troubleshooting

### Tray Icon Issues
1. **Icon Not Circular**: Restart application to refresh icon cache
2. **Colors Not Changing**: Check if visual feedback is enabled in settings
3. **Icon Missing**: Verify `disable_tray = false` in config file

### Notification Issues
1. **No Notifications Appearing**: Check `disable_visual = false` in settings
2. **Notifications Stuck**: Restart application to reset notification system
3. **Performance Issues**: Disable visual feedback if experiencing slowdowns

### Visual Feedback Problems
1. **Notifications Behind Other Windows**: Check Windows always-on-top permissions
2. **Animation Not Smooth**: Ensure sufficient system resources available
3. **Window Positioning Issues**: Restart application to reset window positioning

## 📝 Version History

### v0.3.1 - Enhanced Visual Experience
- ✅ **Circular Tray Icons** - Modern circular design with anti-aliased edges
- ✅ **Visual Foreground Notifications** - Always-on-top status windows during recording/transcribing
- ✅ **Animated Indicators** - Dynamic progress indicators with timer display
- ✅ **Thread-Safe Architecture** - Isolated UI threads for optimal performance
- ✅ **Enhanced User Experience** - Clear visual feedback for all application states
- ✅ **Performance Optimized** - Minimal resource usage for visual features

### Previous Features
- ✅ Complete system tray icon implementation
- ✅ Comprehensive settings window with restart management
- ✅ Performance optimizations (50ms default threshold)
- ✅ Automatic restart functionality

---

*For more information, see the main README.md or check the application logs for detailed operation information.* 