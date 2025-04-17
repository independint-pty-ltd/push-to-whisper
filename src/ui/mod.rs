use anyhow::Result;
use log::{error, info};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use tray_item::{TrayItem, IconSource};
use std::fs;
use std::path::PathBuf;

mod ico_data;
mod settings;

// Re-export settings module functions

// Configuration constants
pub const ENABLE_SYSTEM_TRAY: bool = true;
pub const ENABLE_VISUAL_FEEDBACK: bool = true;

#[derive(Debug, Clone)]
pub struct TrayConfig {
    pub enable_system_tray: bool,
    pub enable_visual_feedback: bool,
}

impl Default for TrayConfig {
    fn default() -> Self {
        Self {
            enable_system_tray: ENABLE_SYSTEM_TRAY,
            enable_visual_feedback: ENABLE_VISUAL_FEEDBACK,
        }
    }
}

// Global state
static TRAY_ICON_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static TRAY_INSTANCE: Lazy<Mutex<Option<TrayItem>>> = Lazy::new(|| Mutex::new(None));

fn save_icon_data(data: &[u8], name: &str) -> Result<PathBuf> {
    let icons_dir = PathBuf::from("icons");
    if !icons_dir.exists() {
        fs::create_dir_all(&icons_dir)?;
    }
    let icon_path = icons_dir.join(name);
    fs::write(&icon_path, data)?;
    Ok(icon_path)
}

pub fn init_tray_icon() -> Result<()> {
    if !ENABLE_SYSTEM_TRAY {
        return Ok(());
    }
    
    // Create the initial icon (blue for normal state)
    let icon = create_icon([30, 144, 255, 255])?;
    
    // Create the menu
    let menu = Menu::new();
    
    // Add menu items
    tray.add_menu_item("Quit", || {
        crate::utils::request_exit();
    })?;
    
    tray.add_menu_item("Toggle Recording", toggle_recording)?;

    *TRAY_INSTANCE.lock() = Some(tray);
    info!("System tray icon initialized");
    Ok(())
}

pub fn update_tray_icon(is_recording: bool) {
    if !ENABLE_SYSTEM_TRAY {
        return;
    }

    if let Some(tray) = TRAY_INSTANCE.lock().as_mut() {
        // Create the icon source based on platform and recording state
        #[cfg(target_os = "windows")]
        let icon_source = {
            // On Windows, use our custom resources defined in resources.rc
            if is_recording {
                IconSource::Resource("recording-icon")
            } else {
                IconSource::Resource("normal-icon")
            }
        };

        #[cfg(target_os = "macos")]
        let icon_source = {
            // On macOS, we use the file path
            let icon_path = if is_recording {
                save_icon_data(ico_data::RECORDING_ICON_DATA, "recording_icon.ico")
            } else {
                save_icon_data(ico_data::NORMAL_ICON_DATA, "normal_icon.ico")
            };

            match icon_path {
                Ok(path) => IconSource::File(path.to_str().unwrap().to_string()),
                Err(e) => {
                    error!("Failed to save icon data: {}", e);
                    return;
                }
            }
        };

        #[cfg(all(target_os = "linux", feature = "ksni"))]
        let icon_source = {
            // On Linux with ksni, we use the file path
            let icon_path = if is_recording {
                save_icon_data(ico_data::RECORDING_ICON_DATA, "recording_icon.ico")
            } else {
                save_icon_data(ico_data::NORMAL_ICON_DATA, "normal_icon.ico")
            };

            match icon_path {
                Ok(path) => IconSource::File(path.to_str().unwrap().to_string()),
                Err(e) => {
                    error!("Failed to save icon data: {}", e);
                    return;
                }
            }
        };

        #[cfg(all(target_os = "linux", feature = "libappindicator"))]
        let icon_source = {
            // On Linux with libappindicator, we use a resource name
            if is_recording {
                IconSource::Resource("media-record")
            } else {
                IconSource::Resource("accessories-calculator")
            }
        };

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            all(target_os = "linux", feature = "ksni"),
            all(target_os = "linux", feature = "libappindicator")
        )))]
        let icon_source = {
            // Fallback for other platforms
            IconSource::Resource("")
        };

        if let Err(e) = tray.set_icon(icon_source) {
            error!("Failed to update tray icon: {}", e);
        }
    }
}

fn toggle_recording() {
    let mut active = TRAY_ICON_ACTIVE.lock();
    *active = !*active;
    
    if *active {
        if let Err(e) = crate::audio::start_recording() {
            error!("Failed to start recording: {}", e);
            *active = false;
        }
    } else {
        if let Err(e) = crate::audio::stop_recording() {
            error!("Failed to stop recording: {}", e);
        }
    }
    
    update_tray_icon(*active);
} 