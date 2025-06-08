use anyhow::{Context, Result};
use log::{error, info, warn, debug};
use once_cell::sync::Lazy;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use tray_icon::{self, Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tray_icon::menu::{Menu, MenuItem, MenuId, MenuEvent};
use std::cell::RefCell;
use parking_lot::Mutex;
use std::time::Duration;



mod ico_data;
mod settings;
// pub mod notification; // Using overlay instead
pub mod overlay;

// Re-export settings module functions for usage elsewhere
pub use settings::open_settings;

// Configuration constants
pub const ENABLE_SYSTEM_TRAY: bool = true;
// Removed unused constant

// Enum for tray icon state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Normal,
    Recording,
    Transcribing,
}

// Event types
#[derive(Debug, Clone)]
pub enum MenuAction {
    OpenSettings,
    ShowAbout,
    Quit,
}

// Thread-local storage for tray icon (since TrayIcon is not Send + Sync)
thread_local! {
    static TRAY_ICON: RefCell<Option<TrayIcon>> = RefCell::new(None);
}

// Global state
static APP_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| Mutex::new(AppState::Normal));
static MENU_CHANNEL: Lazy<Mutex<Option<Sender<MenuAction>>>> = Lazy::new(|| Mutex::new(None));
static ACTION_RECEIVER: Lazy<Mutex<Option<Receiver<MenuAction>>>> = Lazy::new(|| Mutex::new(None));
static ICON_UPDATE_SENDER: Lazy<Mutex<Option<Sender<AppState>>>> = Lazy::new(|| Mutex::new(None));

// Store menu item IDs for later reference
static SETTINGS_MENU_ID: Lazy<Mutex<Option<MenuId>>> = Lazy::new(|| Mutex::new(None));
static ABOUT_MENU_ID: Lazy<Mutex<Option<MenuId>>> = Lazy::new(|| Mutex::new(None));
static EXIT_MENU_ID: Lazy<Mutex<Option<MenuId>>> = Lazy::new(|| Mutex::new(None));

/// Show visual notification window for recording/transcribing
fn show_visual_notification(state: AppState) {
    debug!("show_visual_notification called with state: {:?}", state);
    
    // Check the actual configuration setting instead of hardcoded constant
    let config = crate::utils::get_config();
    if config.disable_visual {
        debug!("Visual feedback is disabled in config, skipping notification");
        return;
    }
    
    debug!("Showing overlay notification for state: {:?}", state);
    
    // Use the new overlay notification system
    overlay::show_overlay(state);
}



/// Initialize system tray icon with context menu
pub fn init_tray_icon() -> Result<()> {
    if !ENABLE_SYSTEM_TRAY {
        info!("System tray disabled, skipping initialization");
        return Ok(());
    }
    
    info!("Initializing system tray icon");
    
    // Channel for our application's MenuActions
    let (action_tx, action_rx): (Sender<MenuAction>, Receiver<MenuAction>) = std::sync::mpsc::channel();
    *MENU_CHANNEL.lock() = Some(action_tx);
    *ACTION_RECEIVER.lock() = Some(action_rx);
    
    // Channel for icon updates
    let (icon_tx, icon_rx): (Sender<AppState>, Receiver<AppState>) = std::sync::mpsc::channel();
    *ICON_UPDATE_SENDER.lock() = Some(icon_tx);
    
    // Start the tray icon thread
    thread::spawn(move || {
        if let Err(e) = run_tray_icon_thread(icon_rx) {
            error!("Tray icon thread error: {}", e);
        }
    });
    
    info!("Successfully started tray icon thread");
    Ok(())
}

/// Run the tray icon in its own thread
fn run_tray_icon_thread(icon_rx: Receiver<AppState>) -> Result<()> {
    // Create the menu
    let menu = Menu::new();
    let settings_item = MenuItem::new("Settings", true, None);
    let about_item = MenuItem::new("About", true, None);
    let separator = MenuItem::new("", false, None);
    let exit_item = MenuItem::new("Exit", true, None);

    // Store menu IDs for event handling
    *SETTINGS_MENU_ID.lock() = Some(settings_item.id().clone());
    *ABOUT_MENU_ID.lock() = Some(about_item.id().clone());
    *EXIT_MENU_ID.lock() = Some(exit_item.id().clone());

    menu.append_items(&[
        &settings_item,
        &about_item,
        &separator,
        &exit_item,
    ]).context("Failed to add menu items")?;

    // Build the tray icon with grey color (normal state)
    let icon_data = create_icon_rgba(128, 128, 128); // Start with grey
    let icon = Icon::from_rgba(icon_data, 16, 16)
        .context("Failed to create icon from RGBA data")?;
    
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Push-to-Whisper - Ready")
        .with_icon(icon)
        .build()
        .context("Failed to build tray icon")?;
        
    // Store the tray icon in thread-local storage
    TRAY_ICON.with(|icon_ref| {
        *icon_ref.borrow_mut() = Some(tray_icon);
    });
    
    // Event handling loop
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    
    loop {
        // Handle menu events
        if let Ok(event) = menu_channel.try_recv() {
            handle_menu_event(event);
        }
        
        // Handle tray icon events (clicks)
        if let Ok(event) = tray_channel.try_recv() {
            handle_tray_event(event);
        }
        
        // Handle icon update requests
        if let Ok(new_state) = icon_rx.try_recv() {
            update_tray_icon_internal(new_state);
        }
        
        thread::sleep(Duration::from_millis(10));
    }
}

/// Handle menu events
fn handle_menu_event(event: MenuEvent) {
    let settings_id = SETTINGS_MENU_ID.lock().clone();
    let about_id = ABOUT_MENU_ID.lock().clone();
    let exit_id = EXIT_MENU_ID.lock().clone();
    
    if let Some(sender) = &*MENU_CHANNEL.lock() {
        let action = if Some(&event.id) == settings_id.as_ref() {
            Some(MenuAction::OpenSettings)
        } else if Some(&event.id) == about_id.as_ref() {
            Some(MenuAction::ShowAbout)
        } else if Some(&event.id) == exit_id.as_ref() {
            Some(MenuAction::Quit)
        } else {
            None
        };
        
        if let Some(action) = action {
            if let Err(e) = sender.send(action) {
                error!("Failed to send menu action: {}", e);
            }
        }
    }
}

/// Handle tray icon events (clicks)
fn handle_tray_event(event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click { button, .. } => {
            match button {
                tray_icon::MouseButton::Left => {
                    // Left click opens settings
                    if let Some(sender) = &*MENU_CHANNEL.lock() {
                        if let Err(e) = sender.send(MenuAction::OpenSettings) {
                            error!("Failed to send settings action: {}", e);
                        }
                    }
                },
                tray_icon::MouseButton::Right => {
                    // Right click shows context menu (handled automatically by tray-icon)
                },
                _ => {}
            }
        },
        _ => {}
    }
}

/// Helper function to create an RGBA buffer for a 16x16 circular icon with the specified color
fn create_icon_rgba(r: u8, g: u8, b: u8) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(16 * 16 * 4);
    let center = 8.0; // Center of 16x16 icon
    let radius = 6.5; // Slightly smaller than half to create nice circular shape
    
    for y in 0..16 {
        for x in 0..16 {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance <= radius {
                // Inside the circle - use the specified color
                rgba.push(r);  // R
                rgba.push(g);  // G
                rgba.push(b);  // B
                rgba.push(255); // A (fully opaque)
            } else if distance <= radius + 1.0 {
                // Anti-aliasing edge - blend with transparency
                let alpha = ((radius + 1.0 - distance) * 255.0) as u8;
                rgba.push(r);  // R
                rgba.push(g);  // G
                rgba.push(b);  // B
                rgba.push(alpha); // A (anti-aliased)
            } else {
                // Outside the circle - transparent
                rgba.push(0);   // R
                rgba.push(0);   // G
                rgba.push(0);   // B
                rgba.push(0);   // A (transparent)
            }
        }
    }
    
    rgba
}

/// Update the tray icon based on application state (called from main thread)
pub fn update_tray_icon(state: AppState) {
    if !ENABLE_SYSTEM_TRAY {
        return;
    }
    
    debug!("Requesting tray icon update to state: {:?}", state);
    
    // Store the current state
    *APP_STATE.lock() = state;
    
    // Show visual notification for recording/transcribing states
    show_visual_notification(state);
    
    // Send update request to tray icon thread
    if let Some(sender) = &*ICON_UPDATE_SENDER.lock() {
        if let Err(e) = sender.send(state) {
            error!("Failed to send icon update request: {}", e);
        }
    }
}

/// Update the tray icon internally (called from tray icon thread)
fn update_tray_icon_internal(state: AppState) {
    debug!("Updating tray icon to state: {:?}", state);
    
    // Get the icon data for the current state
    let icon_data = match state {
        AppState::Normal => {
            debug!("Creating GREY icon for normal state");
            create_icon_rgba(128, 128, 128)      // Grey
        },
        AppState::Recording => {
            debug!("Creating RED icon for recording state");
            create_icon_rgba(220, 20, 20)        // Red
        },
        AppState::Transcribing => {
            debug!("Creating AMBER icon for transcribing state");
            create_icon_rgba(255, 191, 0)        // Amber/Orange
        },
    };
    
    // Create the icon from RGBA data and update
    match Icon::from_rgba(icon_data, 16, 16) {
        Ok(icon) => {
            TRAY_ICON.with(|tray_icon_ref| {
                if let Some(tray_icon) = &*tray_icon_ref.borrow() {
                    if let Err(err) = tray_icon.set_icon(Some(icon)) {
                        error!("Failed to update tray icon: {}", err);
                    } else {
                        debug!("Successfully updated tray icon to state: {:?}", state);
                    }
                } else {
                    warn!("No tray icon found for update!");
                }
            });
        },
        Err(err) => {
            error!("Failed to create icon from RGBA data for state {:?}: {}", state, err);
        }
    }
    
    // Update tooltip based on state
    let tooltip = match state {
        AppState::Normal => "Push-to-Whisper - Ready",
        AppState::Recording => "Push-to-Whisper - Recording...",
        AppState::Transcribing => "Push-to-Whisper - Transcribing...",
    };
    
    TRAY_ICON.with(|tray_icon_ref| {
        if let Some(tray_icon) = &*tray_icon_ref.borrow() {
            if let Err(err) = tray_icon.set_tooltip(Some(tooltip)) {
                error!("Failed to update tooltip: {}", err);
            }
        }
    });
}

// Removed unused recording notification functions

/// Process any pending menu actions
pub fn process_menu_actions() -> Result<bool> {
    if !ENABLE_SYSTEM_TRAY {
        return Ok(false);
    }
    
    // Try to receive a message from our global receiver
    let receiver_lock = ACTION_RECEIVER.lock();
    if let Some(rx) = &*receiver_lock {
        match rx.try_recv() {
            Ok(action) => {
                info!("Received menu action: {:?}", action);
                match action {
                    MenuAction::OpenSettings => {
                        info!("Opening settings window");
                        if let Err(e) = open_settings() {
                            error!("Failed to open settings: {}", e);
                        }
                    },
                    MenuAction::ShowAbout => {
                        info!("Showing about dialog");
                        show_about_dialog();
                    },
                    MenuAction::Quit => {
                        info!("Quit menu action received");
                        return Ok(true); // Signal the main loop to exit
                    },
                }
                return Ok(false);
            },
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // No message available, just continue
            },
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                warn!("Menu action channel disconnected");
            },
        }
    }
    
    Ok(false)
}

/// Cleanup system tray resources
pub fn cleanup_tray() {
    if !ENABLE_SYSTEM_TRAY {
        return;
    }
    
    info!("Cleaning up system tray resources");
    
    // Clear the thread-local tray icon
    TRAY_ICON.with(|icon_ref| {
        *icon_ref.borrow_mut() = None;
    });
}

/// Show an about dialog
fn show_about_dialog() {
    #[cfg(target_os = "windows")]
    {
        let title = "About Push-to-Whisper";
        let message = concat!(
            "Push-to-Whisper v",
            env!("CARGO_PKG_VERSION"),
            "\n\nA fast, private, and efficient push-to-speak transcription tool",
            "\nthat uses OpenAI's Whisper model for real-time speech-to-text.",
            "\n\nCopyright © 2023"
        );
        
        // Use Windows MessageBox - 0 is the MB_OK icon type
        show_message_box(title, message, 0);
    }
}

/// Show a message box (Windows specific)
#[cfg(target_os = "windows")]
pub fn show_message_box(title: &str, message: &str, icon_type: u32) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
    
    let title_cstr = std::ffi::CString::new(title).unwrap();
    let message_cstr = std::ffi::CString::new(message).unwrap();
    
    unsafe {
        MessageBoxA(
            0,
            message_cstr.as_ptr() as *const _,  // Cast to the expected type
            title_cstr.as_ptr() as *const _,
            MB_OK | icon_type
        );
    }
}

#[cfg(not(target_os = "windows"))]
pub fn show_message_box(_title: &str, _message: &str, _icon_type: u32) {
    // Placeholder for non-Windows platforms
}

// Removed unused show_tray_menu function 