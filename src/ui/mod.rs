use anyhow::{Context, Result};
use log::{error, info, warn, debug};
use once_cell::sync::{Lazy, OnceCell};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::utils::{self};

mod ico_data;
mod settings;

// Re-export settings module functions for usage elsewhere
pub use settings::open_settings;

// Configuration constants
pub const ENABLE_SYSTEM_TRAY: bool = true;
pub const ENABLE_VISUAL_FEEDBACK: bool = true;

// Enum for tray icon state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Normal,
    Recording,
    Transcribing,
}

// Event types
#[derive(Debug)]
enum MenuAction {
    ToggleRecording,
    OpenSettings,
    Quit,
}

/// Initialize system tray icon (stubbed)
pub fn init_tray_icon() -> Result<()> {
    info!("System tray functionality is disabled in this build");
    Ok(())
}

/// Update the tray icon based on application state (stubbed)
pub fn update_tray_icon(state: AppState) {
    debug!("Would update tray icon to state: {:?}", state);
}

/// Show a message box (Windows specific)
#[cfg(target_os = "windows")]
pub fn show_message_box(title: &str, message: &str, icon: u32) {
    info!("Would show message box: {} - {}", title, message);
}

#[cfg(not(target_os = "windows"))]
pub fn show_message_box(_title: &str, _message: &str, _icon: u32) {
    // Placeholder for non-Windows platforms
} 