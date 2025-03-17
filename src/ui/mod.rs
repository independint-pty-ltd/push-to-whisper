use anyhow::Result;
use log::{error, info};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::cell::RefCell;

// Tray icon imports
use tray_icon::{Icon, TrayIconBuilder, menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}};

mod ico_data;
mod settings;

// Re-export settings module functions
pub use settings::{open_settings, close_settings, is_settings_window_open};

// Configuration constants
pub const ENABLE_SYSTEM_TRAY: bool = true;
pub const ENABLE_VISUAL_FEEDBACK: bool = true;

// Windows system icon constants
#[cfg(target_os = "windows")]
const IDI_APPLICATION: &str = "32512"; // Standard application icon
#[cfg(target_os = "windows")]
const IDI_ERROR: &str = "32513";       // Error icon (red X)
#[cfg(target_os = "windows")]
const IDI_WARNING: &str = "32515";     // Warning icon (yellow triangle)
#[cfg(target_os = "windows")]
const IDI_INFORMATION: &str = "32516"; // Information icon (blue i)

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

// Thread-local storage for tray icon
thread_local! {
    static TRAY_ICON: RefCell<Option<tray_icon::TrayIcon>> = RefCell::new(None);
}

// Global state
static CURRENT_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| Mutex::new(AppState::Normal));

// Define an enum for the application state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Normal,
    Recording,
    Transcribing,
}

// Create a simple colored icon
fn create_icon(color: [u8; 4]) -> Result<Icon> {
    // Create a 32x32 icon
    let width = 32;
    let height = 32;
    
    // Create RGBA data for the icon
    let mut rgba = Vec::with_capacity(width * height * 4);
    
    for y in 0..height {
        for x in 0..width {
            // Calculate distance from center for a circular icon
            let center_x = width as f32 / 2.0;
            let center_y = height as f32 / 2.0;
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // Create a filled circle with a border
            if distance < 14.0 {
                // Inside the circle - use the specified color
                rgba.extend_from_slice(&color);
            } else if distance < 16.0 {
                // Border - use black for contrast
                rgba.extend_from_slice(&[0, 0, 0, 255]);
            } else {
                // Outside the circle - transparent
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    
    // Create the icon from RGBA data
    Icon::from_rgba(rgba, width as u32, height as u32)
        .map_err(|e| anyhow::anyhow!("Failed to create icon: {}", e))
}

pub fn init_tray_icon() -> Result<()> {
    if !ENABLE_SYSTEM_TRAY {
        return Ok(());
    }
    
    // Create the initial icon (blue for normal state)
    let icon = create_icon([30, 144, 255, 255])?;
    
    // Create the menu
    let mut menu = Menu::new();
    
    // Add menu items
    let settings_item = MenuItem::new("Open Settings", true, None);
    let settings_id = settings_item.id().clone();
    menu.append(&settings_item)?;
    
    let updates_item = MenuItem::new("Check for Updates", true, None);
    let updates_id = updates_item.id().clone();
    menu.append(&updates_item)?;
    
    menu.append(&PredefinedMenuItem::separator())?;
    
    let toggle_item = MenuItem::new("Toggle Recording", true, None);
    let toggle_id = toggle_item.id().clone();
    menu.append(&toggle_item)?;
    
    menu.append(&PredefinedMenuItem::separator())?;
    
    let quit_item = MenuItem::new("Quit", true, None);
    let quit_id = quit_item.id().clone();
    menu.append(&quit_item)?;
    
    // Create the tray icon
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("Push to Whisper")
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .build()?;
    
    // Store the tray icon in thread-local storage
    TRAY_ICON.with(|cell| {
        *cell.borrow_mut() = Some(tray_icon);
    });
    
    // Handle menu events
    let menu_channel = MenuEvent::receiver();
    std::thread::spawn(move || {
        loop {
            if let Ok(event) = menu_channel.recv() {
                info!("Menu event received: {:?}", event);
                
                // Check the event ID against our stored IDs
                if event.id == settings_id {
                    if let Err(e) = settings::open_settings() {
                        error!("Failed to open settings: {}", e);
                    }
                } else if event.id == updates_id {
                    info!("Check for updates clicked");
                } else if event.id == toggle_id {
                    toggle_recording();
                } else if event.id == quit_id {
                    crate::utils::request_exit();
                }
            }
        }
    });
    
    // Handle tray icon events
    let tray_channel = tray_icon::TrayIconEvent::receiver();
    std::thread::spawn(move || {
        loop {
            if let Ok(event) = tray_channel.recv() {
                info!("Tray icon event: {:?}", event);
                
                // Check for click events - match on the pattern correctly
                if matches!(event, tray_icon::TrayIconEvent::Click { .. }) {
                    toggle_recording();
                }
            }
        }
    });
    
    info!("System tray icon initialized with menu");
    
    // Set initial state
    update_tray_icon(AppState::Normal);
    
    Ok(())
}

pub fn update_tray_icon(state: AppState) {
    if !ENABLE_SYSTEM_TRAY {
        return;
    }
    
    // Check if the state has actually changed
    let current_state;
    {
        let state_guard = CURRENT_STATE.lock();
        current_state = *state_guard;
    }
    
    // Only update if the state has changed
    if state == current_state {
        // State hasn't changed, no need to update
        return;
    }
    
    // Update the current state
    {
        let mut current = CURRENT_STATE.lock();
        *current = state;
    }
    
    // Choose color based on state
    let color = match state {
        AppState::Recording => {
            [255, 0, 0, 255]  // Bright red
        },
        AppState::Transcribing => {
            [255, 165, 0, 255]  // Orange
        },
        AppState::Normal => {
            [30, 144, 255, 255]  // Dodger blue
        },
    };
    
    // Create the new icon
    match create_icon(color) {
        Ok(new_icon) => {
            // Update the icon in the main thread
            TRAY_ICON.with(|cell| {
                if let Some(tray) = &*cell.borrow() {
                    if let Err(e) = tray.set_icon(Some(new_icon)) {
                        error!("Failed to update tray icon: {}", e);
                    } else {
                        info!("Updated tray icon to state: {:?}", state);
                        
                        // Update tooltip
                        let status = match state {
                            AppState::Recording => "Recording",
                            AppState::Transcribing => "Transcribing",
                            AppState::Normal => "Idle",
                        };
                        let version = env!("CARGO_PKG_VERSION");
                        let tooltip = format!("Push to Whisper v{} - {}", version, status);
                        
                        if let Err(e) = tray.set_tooltip(Some(&tooltip)) {
                            error!("Failed to update tooltip: {}", e);
                        }
                    }
                } else {
                    error!("Tray icon not found when trying to update");
                }
            });
        },
        Err(e) => {
            error!("Failed to create icon: {}", e);
        }
    }
}

fn toggle_recording() {
    // Check current recording state
    let is_recording = crate::audio::is_recording();
    
    if is_recording {
        // Currently recording, stop it
        info!("Stopping recording from menu click");
        if let Err(e) = crate::audio::stop_recording() {
            error!("Failed to stop recording: {}", e);
        } else {
            // Force update the icon to Normal state by first setting it to a different state
            {
                let mut current = CURRENT_STATE.lock();
                *current = AppState::Recording; // Set to opposite state to force update
            }
            update_tray_icon(AppState::Normal);
        }
    } else {
        // Not recording, start it
        info!("Starting recording from menu click");
        if let Err(e) = crate::audio::start_recording() {
            error!("Failed to start recording: {}", e);
        } else {
            // Force update the icon to Recording state by first setting it to a different state
            {
                let mut current = CURRENT_STATE.lock();
                *current = AppState::Normal; // Set to opposite state to force update
            }
            update_tray_icon(AppState::Recording);
        }
    }
} 