# System Responsiveness Fix for Transcription

## Overview
This fix addresses the mouse lag and system slowness during transcription by implementing CPU throttling and configurable thread priorities.

## Key Changes

### 1. Thread Priority System
- Changed default thread priority from `THREAD_PRIORITY_BELOW_NORMAL` to `THREAD_PRIORITY_LOWEST`
- Added configurable transcription priority levels:
  - **low** (default): Uses `THREAD_PRIORITY_LOWEST` for minimal system impact
  - **normal**: Uses `THREAD_PRIORITY_BELOW_NORMAL` for balanced performance
  - **high**: Uses `THREAD_PRIORITY_NORMAL` for faster transcription

### 2. CPU Yielding
Added a progress callback to the Whisper processing that:
- Calls `thread::yield_now()` to give other threads CPU time
- Adds a 100 microsecond sleep to reduce CPU usage
- Ensures the system remains responsive during transcription

### 3. Configuration Options

#### Config File Setting
```ini
# Transcription thread priority (low, normal, high)
transcription_priority = low
```

#### Command Line Arguments
```bash
# Set transcription priority
push-to-whisper.exe --transcription-priority low
push-to-whisper.exe --tp low  # Short form
```

## How It Works

1. **Low Priority (Default)**:
   - Transcription runs at the lowest thread priority
   - Minimal impact on mouse movement and system UI
   - Transcription may take slightly longer but system remains fully responsive

2. **Normal Priority**:
   - Balanced approach between speed and responsiveness
   - Good for users with powerful systems

3. **High Priority**:
   - Faster transcription but may cause some system lag
   - Use only if transcription speed is critical

## Expected Results

With the default "low" priority setting:
- Mouse movement should remain smooth during transcription
- System UI should stay responsive
- Transcription may take 10-20% longer but won't freeze the system
- CPU usage will be more evenly distributed

## Recommendations

1. Start with the default "low" priority setting
2. If transcription is too slow, try "normal" priority
3. Only use "high" priority if you have a powerful system and need fastest transcription
4. Consider using a smaller model (tiny.en or base.en) for better overall performance