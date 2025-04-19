# Push-to-Whisper System Tray Enhancement Plan

## Overview
This document outlines the plan to enhance the system tray functionality in Push-to-Whisper. The current implementation has stubbed functions for the system tray but no active implementation. We will implement a proper Windows system tray with status indicators and a settings menu.

## Requirements
1. **Dynamic Status Indicator**: System tray icon that changes based on application state
   - Normal state (passive/idle)
   - Recording state (when actively recording)
   - Transcribing state (when processing audio)

2. **Context Menu**: Clickable options when right-clicking the tray icon
   - Settings
   - About
   - Quit

3. **Left-Click Action**: Open settings when left-clicking the tray icon

4. **Windows Specific**: Implement using Windows-specific APIs

## Technical Implementation

### 1. System Tray Icon

#### 1.1 Icon States
- Define three distinct icons for the different states:
  - Normal icon (gray/inactive)
  - Recording icon (red/active)
  - Transcribing icon (blue/processing)

#### 1.2 Icon Implementation
- We already have `NORMAL_ICON_DATA` and `RECORDING_ICON_DATA` in `ico_data.rs`
- Add `TRANSCRIBING_ICON_DATA` for the transcribing state
- Use `tray-icon` crate for Windows system tray functionality

### 2. Context Menu

#### 2.1 Menu Items
- **Settings**: Opens the settings dialog
- **About**: Shows information about the application
- **Quit**: Exits the application

#### 2.2 Menu Implementation
- Use Windows-specific menu APIs via the `tray-icon` crate
- Connect menu actions to existing functionality:
  - Settings → `ui::open_settings()`
  - About → New function to display about dialog
  - Quit → Set `EXIT_REQUESTED` atomic to true

### 3. UI Integration

#### 3.1 State Management
- Update `AppState` enum in `ui/mod.rs` to include all states
- Modify `update_tray_icon()` to actually update the icon based on state
- Implement proper icon loading and registration with Windows

#### 3.2 Event Handling
- Set up message loop for tray icon events
- Handle left-click and right-click events
- Connect context menu actions to application functions

### 4. Implementation Plan

1. **Phase 1: Icon Implementation**
   - Add transcribing icon data to `ico_data.rs`
   - Implement actual tray icon functionality in `init_tray_icon()`
   - Implement proper icon updating in `update_tray_icon()`

2. **Phase 2: Context Menu**
   - Implement context menu structure
   - Connect menu actions to application functions
   - Add about dialog implementation

3. **Phase 3: Event Handling**
   - Set up message loop for icon events
   - Implement left-click handling to open settings
   - Test and refine interactions

## Code Structure Changes

### New Files to Create
- None needed, we'll enhance existing files

### Existing Files to Modify

1. **src/ui/mod.rs**
   - Replace stubbed functions with actual implementations:
     - `init_tray_icon()`
     - `update_tray_icon()`
   - Add functions for tray menu handling

2. **src/ui/ico_data.rs**
   - Add `TRANSCRIBING_ICON_DATA` for the transcribing state

3. **src/main.rs**
   - Update the main event loop to properly handle tray events
   - Ensure proper cleanup of tray resources on exit

## Dependencies
- Already using `tray-icon = "0.20.0"` in Cargo.toml
- Already using `windows-sys` for Windows-specific functionality

## Testing Plan
1. Test icon state changes during:
   - Normal operation
   - Recording (holding down key)
   - Transcribing (after key release)
   
2. Test context menu functionality:
   - Settings opens settings window
   - About shows about dialog
   - Quit properly exits application
   
3. Test left-click opens settings dialog

## Future Enhancements (Post-Implementation)
1. Add notification support (Windows toast notifications)
2. Add keyboard shortcut indicators in the menu
3. Add option to launch at system startup 