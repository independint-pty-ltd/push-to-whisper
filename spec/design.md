# Push-to-Whisper Technical Design Document

## Table of Contents
1. [System Overview](#system-overview)
2. [Architecture Design](#architecture-design)
3. [Component Design](#component-design)
4. [Data Flow](#data-flow)
5. [Threading Model](#threading-model)
6. [Performance Design](#performance-design)
7. [Security Design](#security-design)
8. [Error Handling](#error-handling)
9. [Testing Strategy](#testing-strategy)
10. [Deployment Design](#deployment-design)

## System Overview

### Design Philosophy
Push-to-Whisper follows these core design principles:
- **Performance First**: Sub-second response times are non-negotiable
- **Privacy by Design**: No data leaves the user's machine
- **Fail Gracefully**: Errors should never crash the application
- **Minimal Dependencies**: Only essential external libraries
- **User Transparency**: Clear feedback at every step

### Technology Stack
- **Language**: Rust (for memory safety and performance)
- **AI Model**: OpenAI Whisper (via whisper-rs bindings)
- **Audio**: CPAL (cross-platform audio layer)
- **UI Framework**: egui (immediate mode GUI)
- **System Integration**: Windows APIs via winapi-rs

## Architecture Design

### High-Level Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                     User Applications                        │
└─────────────────┬───────────────────────┬───────────────────┘
                  │                       │
                  ▼                       ▼
┌─────────────────────────┐     ┌─────────────────────────┐
│   Text Insertion API    │     │    System Tray Icon     │
└────────────┬────────────┘     └────────────┬────────────┘
             │                                │
             ▼                                ▼
┌─────────────────────────────────────────────────────────────┐
│                    Core Application Engine                   │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐   │
│  │   Hotkey    │  │    Audio     │  │   Transcription │   │
│  │   Handler   │  │   Capture    │  │     Engine      │   │
│  └─────────────┘  └──────────────┘  └─────────────────┘   │
└─────────────────────────────────────────────────────────────┘
             │                                │
             ▼                                ▼
┌─────────────────────────┐     ┌─────────────────────────┐
│    Whisper AI Model     │     │   Configuration Store   │
└─────────────────────────┘     └─────────────────────────┘
```

### Module Structure
```
push-to-whisper/
├── src/
│   ├── main.rs              # Application entry point
│   ├── state.rs             # Global state management
│   ├── error.rs             # Error types and handling
│   │
│   ├── audio/               # Audio subsystem
│   │   ├── mod.rs          # Audio module interface
│   │   ├── capture.rs      # Microphone recording
│   │   ├── playback.rs     # Beep sound generation
│   │   └── keepalive.rs    # Headphone keepalive
│   │
│   ├── whisper/            # AI transcription
│   │   ├── mod.rs          # Whisper module interface
│   │   ├── model.rs        # Model loading and management
│   │   ├── transcribe.rs   # Transcription engine
│   │   └── cuda.rs         # GPU acceleration handling
│   │
│   ├── input/              # Input/Output handling
│   │   ├── mod.rs          # Input module interface
│   │   ├── keyboard.rs     # Hotkey detection
│   │   ├── text_insert.rs  # Text insertion strategies
│   │   └── clipboard.rs    # Clipboard management
│   │
│   ├── ui/                 # User interface
│   │   ├── mod.rs          # UI module interface
│   │   ├── tray.rs         # System tray integration
│   │   ├── settings.rs     # Settings window
│   │   └── overlay.rs      # Visual notifications
│   │
│   ├── model/              # Model management
│   │   ├── mod.rs          # Model module interface
│   │   └── download.rs     # Model downloading
│   │
│   └── utils/              # Utilities
│       ├── mod.rs          # Utils module interface
│       ├── config.rs       # Configuration management
│       └── instance.rs     # Single instance enforcement
```

## Component Design

### 1. Audio Capture Component

**Purpose**: Efficiently capture microphone audio with minimal latency

**Design Decisions**:
- Use lock-free ring buffer for audio samples
- 16kHz sampling rate (Whisper's native rate)
- Mono channel to reduce data size
- 5-minute circular buffer capacity
- Non-blocking audio callbacks

**Interface**:
```rust
pub trait AudioCapture {
    fn start_recording(&mut self) -> Result<()>;
    fn stop_recording(&mut self) -> Result<Vec<f32>>;
    fn is_recording(&self) -> bool;
}
```

### 2. Transcription Engine

**Purpose**: Convert audio to text using Whisper AI

**Design Decisions**:
- Lazy model loading on first use
- GPU acceleration with CPU fallback
- Separate thread for transcription
- Configurable model sizes
- In-memory processing only

**Key Features**:
- Automatic language detection (large model only)
- Context-aware transcription
- Punctuation and capitalization
- Background noise handling

### 3. Hotkey Handler

**Purpose**: Detect and respond to keyboard events

**Design Decisions**:
- Global keyboard hook via Windows API
- Configurable activation key
- Long-press detection with threshold
- Non-blocking event processing
- Double-ESC exit mechanism

**State Machine**:
```
IDLE ─────[Key Down]────▶ PRESSED ────[Threshold Met]────▶ RECORDING
 ▲                           │                                  │
 │                           │                                  │
 └──────[Key Up Early]───────┘                                  │
 │                                                              │
 └──────────────────[Key Up + Transcribe Complete]─────────────┘
```

### 4. Text Insertion Manager

**Purpose**: Insert transcribed text at cursor position

**Design Decisions**:
- Multiple insertion strategies
- Automatic method selection
- Clipboard preservation
- Special character handling
- Application compatibility layer

**Insertion Methods**:
1. **Clipboard Method** (Primary)
   - Save current clipboard
   - Copy text to clipboard
   - Send Ctrl+V
   - Restore original clipboard

2. **Keyboard Shortcut Method** (Fallback)
   - Direct key sequence simulation
   - Handles special characters

3. **Direct Typing Method** (Last Resort)
   - Character-by-character input
   - Slowest but most compatible

### 5. UI Components

#### System Tray Icon
- **States**: Ready (grey), Recording (red), Transcribing (orange)
- **Actions**: Left-click (settings), Right-click (menu)
- **Updates**: Real-time state reflection

#### Settings Window
- **Framework**: egui for immediate mode GUI
- **Categories**: General, Audio, AI Model, Advanced
- **Features**: Real-time preview, validation, tooltips

#### Overlay Notifications
- **Position**: Bottom-right corner
- **Size**: 250x60 pixels
- **Behavior**: Non-interactive, auto-dismiss
- **Animation**: Fade in/out effects

## Data Flow

### Recording Flow
```
1. User presses hotkey
   └─▶ Hotkey handler detects press
       └─▶ Start threshold timer
           └─▶ Threshold met
               └─▶ Start audio recording
                   └─▶ Update UI state
                       └─▶ Play start beep

2. Audio samples arrive
   └─▶ Write to ring buffer
       └─▶ Monitor buffer health
           └─▶ Handle overflow if needed

3. User releases hotkey
   └─▶ Stop audio recording
       └─▶ Extract audio data
           └─▶ Play stop beep
               └─▶ Submit to transcription
```

### Transcription Flow
```
1. Audio data received
   └─▶ Validate audio format
       └─▶ Convert to Whisper format
           └─▶ Load model if needed
               └─▶ Process with Whisper
                   └─▶ Post-process text
                       └─▶ Insert at cursor
```

## Threading Model

### Thread Architecture
```
Main Thread (UI)
├─▶ System Tray Icon
├─▶ Settings Window
└─▶ Event Loop

Audio Thread (Real-time)
├─▶ Audio Capture
├─▶ Ring Buffer Management
└─▶ Beep Playback

Keyboard Thread
├─▶ Global Hook
├─▶ Hotkey Detection
└─▶ Event Dispatch

Transcription Thread Pool
├─▶ Worker 1: Whisper Processing
├─▶ Worker 2: Model Loading
└─▶ Worker 3: Text Post-processing

Background Threads
├─▶ Headphone Keepalive
├─▶ Configuration Watcher
└─▶ Notification Renderer
```

### Synchronization Strategy
- **State**: Atomic flags for recording/transcribing states
- **Audio Buffer**: Lock-free SPSC ring buffer
- **Commands**: MPSC channels for cross-thread communication
- **UI Updates**: Event-driven message passing
- **Shared Data**: Arc<Mutex<T>> for configuration

## Performance Design

### Optimization Strategies

#### 1. Audio Pipeline
- **Zero-copy audio path** where possible
- **Pre-allocated buffers** to avoid allocations
- **SIMD operations** for audio processing
- **Optimal buffer sizes** (512 samples)

#### 2. Transcription
- **GPU acceleration** when available
- **Model caching** in memory
- **Batch processing** for efficiency
- **Thread pool** for parallel work

#### 3. UI Responsiveness
- **Async operations** for all I/O
- **Non-blocking UI updates**
- **Debounced event handling**
- **Lazy rendering** for efficiency

### Memory Management
- **Fixed-size buffers** for audio (5 minutes max)
- **Model memory mapping** for fast loading
- **Aggressive cleanup** after transcription
- **Memory pool** for frequent allocations

### Latency Targets
- **Hotkey Response**: <10ms
- **Recording Start**: <50ms
- **Transcription Start**: <100ms
- **Text Insertion**: <50ms
- **Total End-to-End**: <500ms perceived

## Security Design

### Threat Model
1. **Audio Eavesdropping**: Prevented by in-memory processing
2. **Keystroke Logging**: Limited to specific hotkey only
3. **Data Exfiltration**: No network access during operation
4. **Privilege Escalation**: Runs with user privileges only

### Security Measures
- **No File Storage**: Audio never written to disk
- **Memory Wiping**: Secure erasure of audio buffers
- **Input Validation**: All user input sanitized
- **Code Signing**: Executable digitally signed
- **Dependency Auditing**: Regular security reviews

## Error Handling

### Error Categories
1. **Recoverable Errors**
   - Audio device unavailable → Retry with default
   - Transcription timeout → Return partial result
   - GPU initialization failure → Fall back to CPU

2. **User Errors**
   - No microphone → Clear error message
   - Model not found → Automatic download
   - Permission denied → Installation guide

3. **Fatal Errors**
   - Memory allocation failure → Graceful shutdown
   - Corrupted model → Re-download prompt
   - System API failure → Error report

### Error Recovery Strategy
```rust
enum RecoveryAction {
    Retry { max_attempts: u32 },
    Fallback { alternative: Strategy },
    UserIntervention { message: String },
    Shutdown { save_state: bool },
}
```

## Testing Strategy

### Unit Testing
- **Audio Processing**: Mock audio devices
- **Transcription**: Test with known audio samples
- **Text Insertion**: Mock system APIs
- **Configuration**: Test parsing and validation

### Integration Testing
- **End-to-End Flow**: Record → Transcribe → Insert
- **Hardware Variations**: Different audio devices
- **OS Compatibility**: Windows 10 vs 11
- **Performance**: Benchmark critical paths

### User Testing
- **Usability Studies**: First-time user experience
- **Accessibility Testing**: Screen reader compatibility
- **Stress Testing**: Extended usage sessions
- **Compatibility Testing**: Various applications

## Deployment Design

### Build Pipeline
```
1. Compile Release Build
   └─▶ Optimize for size and speed
       └─▶ Link statically where possible

2. Code Signing
   └─▶ Sign with certificate
       └─▶ Timestamp signature

3. Package Creation
   └─▶ Single executable
       └─▶ Embedded resources
           └─▶ Compression

4. Distribution
   └─▶ GitHub Releases
       └─▶ Auto-update check
```

### Installation Process
1. **First Run Detection**
2. **Model Download** (if needed)
3. **Permission Requests**
4. **Configuration Generation**
5. **Tray Icon Registration**

### Update Mechanism
- **Version Check**: On startup (configurable)
- **Delta Updates**: Download only changes
- **Rollback Support**: Keep previous version
- **Silent Updates**: With user consent

## Future Considerations

### Scalability
- **Plugin Architecture**: For custom processors
- **Multi-Model Support**: Different AI models
- **Cloud Integration**: Optional sync features
- **API Layer**: For automation tools

### Platform Expansion
- **macOS**: Using native APIs
- **Linux**: X11/Wayland support
- **Mobile Companion**: Remote control app

### Feature Extensions
- **Voice Commands**: "Start recording"
- **Custom Vocabularies**: Technical terms
- **Real-time Preview**: During recording
- **Translation Mode**: Cross-language support

---

*This design document is a living document and will be updated as the architecture evolves. All design decisions should prioritize user experience, performance, and privacy.*