use anyhow::{Context, Result};
use log::{error, info, warn, debug};
use once_cell::sync::{Lazy, OnceCell};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tray_icon::{self, Icon, TrayIcon, TrayIconBuilder};
use tray_icon::menu::{Menu, MenuItem, MenuId};
use winit::event_loop::{EventLoop};
use winit::window::Window;
use winit::event::Event;
use winit::window::WindowAttributes;
use std::sync::Arc;
use std::cell::RefCell;
use parking_lot::Mutex;
use std::time::Duration;

use crate::utils::{self};
use crate::input;

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
#[derive(Debug, Clone)]
pub enum MenuAction {
    OpenSettings,
    ShowAbout,
    Quit,
}

// Thread-local UI elements (since TrayIcon is not Send + Sync)
thread_local! {
    static TRAY_ICON: RefCell<Option<TrayIcon>> = RefCell::new(None);
    static TRAY_MENU: RefCell<Option<Menu>> = RefCell::new(None);
}

// Thread-safe globals
static APP_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| Mutex::new(AppState::Normal));
static MENU_CHANNEL: Lazy<Mutex<Option<Sender<MenuAction>>>> = Lazy::new(|| Mutex::new(None));
static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);

// Store menu item IDs for later reference
static SETTINGS_ID: Lazy<Mutex<Option<MenuId>>> = Lazy::new(|| Mutex::new(None));
static EXIT_ID: Lazy<Mutex<Option<MenuId>>> = Lazy::new(|| Mutex::new(None));

// Helper function to get current time in milliseconds
fn current_time_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Initialize system tray icon with context menu
pub fn init_tray_icon() -> Result<()> {
    if !ENABLE_SYSTEM_TRAY {
        info!("System tray disabled, skipping initialization");
        return Ok(());
    }
    
    info!("Initializing system tray icon");
    
    // Channel for our application's MenuActions (used by show_tray_menu)
    let (action_tx, action_rx): (Sender<MenuAction>, Receiver<MenuAction>) = std::sync::mpsc::channel();
    *MENU_CHANNEL.lock() = Some(action_tx);
    
    // Store the MenuAction receiver for process_menu_actions
    static ACTION_RECEIVER: Lazy<Mutex<Option<Receiver<MenuAction>>>> = Lazy::new(|| Mutex::new(None));
    *ACTION_RECEIVER.lock() = Some(action_rx);
    
    // Create the menu (even though events aren't handled by tray-icon directly)
    let mut menu = Menu::new();
    let settings_item = MenuItem::new("Settings", true, None);
    let exit_item = MenuItem::new("Exit", true, None);
    let separator = MenuItem::new("", false, None);

    menu.append_items(&[
        &settings_item,
        &separator,
        &exit_item,
    ]).context("Failed to add menu items")?;

    // Build the tray icon
    let icon_data = create_icon_rgba(128, 128, 128); // Start with gray (default)
    let icon = Icon::from_rgba(icon_data.clone(), 16, 16)
        .context("Failed to create icon from RGBA data")?;
    
    let tray_icon_builder = TrayIconBuilder::new()
        .with_menu(Box::new(menu)) // Attach the menu visually
        .with_tooltip("Push-to-Whisper")
        .with_icon(icon)
        .with_menu_on_left_click(false); // Don't show default menu
        
    // Build and store the tray icon
    match tray_icon_builder.build() {
        Ok(tray_icon) => {
            info!("Successfully created system tray icon (menu events handled manually)");
            
            // Store in thread-local
            TRAY_ICON.with(|tray_icon_ref| {
                *tray_icon_ref.borrow_mut() = Some(tray_icon);
            });
            
            // No event handling thread needed here
            
            Ok(())
        },
        Err(e) => {
            error!("Failed to create tray icon: {}", e);
            Err(anyhow::anyhow!("Failed to create tray icon: {}", e))
        }
    }
}

/// Helper function to create an RGBA buffer for a 16x16 icon with the specified color
fn create_icon_rgba(r: u8, g: u8, b: u8) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(16 * 16 * 4);
    
    for _y in 0..16 {
        for _x in 0..16 {
            rgba.push(r);  // R
            rgba.push(g);  // G
            rgba.push(b);  // B
            rgba.push(255); // A (fully opaque)
        }
    }
    
    rgba
}

/// Update the tray icon based on application state
pub fn update_tray_icon(state: AppState) {
    if !ENABLE_SYSTEM_TRAY {
        return;
    }
    
    debug!("[Thread {:?}] Attempting to update tray icon to state: {:?}", std::thread::current().id(), state);
    
    // Store the current state
    *APP_STATE.lock() = state;
    
    // Get the icon data for the current state
    let icon_data = match state {
        AppState::Normal => {
            debug!("[Thread {:?}] Creating GRAY icon", std::thread::current().id());
            create_icon_rgba(128, 128, 128)      // Gray
        },
        AppState::Recording => {
            debug!("[Thread {:?}] Creating RED icon", std::thread::current().id());
            create_icon_rgba(220, 0, 0)       // Red (changed from Green)
        },
        AppState::Transcribing => {
            debug!("[Thread {:?}] Creating BLUE icon", std::thread::current().id());
            create_icon_rgba(0, 0, 200)    // Blue
        },
    };
    
    // Create the icon from RGBA data
    match Icon::from_rgba(icon_data.clone(), 16, 16) {
        Ok(icon) => {
            TRAY_ICON.with(|tray_icon_ref| {
                let borrow = tray_icon_ref.borrow();
                if let Some(tray_icon) = &*borrow {
                    debug!("[Thread {:?}] Found tray icon, setting icon for state: {:?}", std::thread::current().id(), state);
                    if let Err(err) = tray_icon.set_icon(Some(icon.clone())) {
                        error!("Failed to update tray icon: {}", err);
                    } else {
                        debug!("[Thread {:?}] Successfully set tray icon to state: {:?}", std::thread::current().id(), state);
                    }
                } else {
                    // Log specifically when the icon is None in thread-local
                    warn!("[Thread {:?}] No tray icon found in thread-local storage for update!", std::thread::current().id());
                }
            });
        },
        Err(err) => {
            error!("Failed to create icon from RGBA data for state {:?}: {}", state, err);
        }
    }
    
    // Update tooltip
    let tooltip = match state {
        AppState::Normal => "Push-to-Whisper (Idle)",
        AppState::Recording => "Push-to-Whisper (Recording...)",
        AppState::Transcribing => "Push-to-Whisper (Transcribing...)",
    };
    
    TRAY_ICON.with(|tray_icon_ref| {
         let borrow = tray_icon_ref.borrow();
         if let Some(tray_icon) = &*borrow {
            if let Err(err) = tray_icon.set_tooltip(Some(tooltip.to_string())) {
                error!("Failed to update tooltip: {}", err);
            } else {
                debug!("[Thread {:?}] Updated tooltip to: {}", std::thread::current().id(), tooltip);
            }
        } else {
             warn!("[Thread {:?}] No tray icon found in thread-local storage for tooltip update!", std::thread::current().id());
        }
    });
    
    // If recording just started, show notification
    if state == AppState::Recording {
        show_recording_notification();
    }
}

/// Show notification that recording has started
#[cfg(target_os = "windows")]
fn show_recording_notification() {
    use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxA;
    
    info!("Showing recording notification");
    
    // Just update the tooltip with a stronger message
    TRAY_ICON.with(|tray_icon_ref| {
        let borrow = tray_icon_ref.borrow();
        if let Some(tray_icon) = &*borrow {
            // Use a distinct tooltip to indicate recording
            if let Err(err) = tray_icon.set_tooltip(Some("⚠️ RECORDING IN PROGRESS ⚠️".to_string())) {
                error!("Failed to update tooltip for notification: {}", err);
            }
        }
    });
    
    // Optional: Use MessageBoxA for a temporary dialog
    #[cfg(feature = "notify-dialog")]
    {
        let title_cstr = std::ffi::CString::new("Push-to-Whisper").unwrap();
        let message_cstr = std::ffi::CString::new("Recording started...").unwrap();
        
        unsafe {
            MessageBoxA(
                0,
                message_cstr.as_ptr() as *const _,
                title_cstr.as_ptr() as *const _,
                0
            );
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn show_recording_notification() {
    // Placeholder for non-Windows platforms
    info!("Recording notification (non-Windows platform)");
}

/// Process any pending menu actions
pub fn process_menu_actions() -> Result<bool> {
    if !ENABLE_SYSTEM_TRAY {
        return Ok(false);
    }
    
    // Access the static receiver
    static ACTION_RECEIVER: Lazy<Mutex<Option<Receiver<MenuAction>>>> = 
        Lazy::new(|| Mutex::new(None));
    
    // Try to receive a message
    let mut receiver_lock = ACTION_RECEIVER.lock();
    if let Some(rx) = &*receiver_lock {
        match rx.try_recv() {
            Ok(action) => {
                info!("Received menu action in main thread: {:?}", action);
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
                        info!("Quit menu action received in main thread");
                        return Ok(true); // Signal the main loop to exit
                    },
                }
                return Ok(false);
            },
            Err(mpsc::TryRecvError::Empty) => {
                // No message available, just continue
            },
            Err(mpsc::TryRecvError::Disconnected) => {
                // Channel disconnected, recreate it next time
                warn!("Menu action channel disconnected, will recreate");
                *receiver_lock = None;
            },
        }
    }
    
    // T key trigger removed as requested
    
    Ok(false)
}

/// Cleanup system tray resources
pub fn cleanup_tray() {
    if !ENABLE_SYSTEM_TRAY {
        return;
    }
    
    info!("Cleaning up system tray resources");
    
    // Use thread-local storage
    TRAY_ICON.with(|icon| {
        *icon.borrow_mut() = None;
    });
    
    TRAY_MENU.with(|menu| {
        *menu.borrow_mut() = None;
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
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK, MB_ICONINFORMATION};
    
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

// Add function to show the tray menu
#[cfg(target_os = "windows")]
fn show_tray_menu() {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreatePopupMenu, TrackPopupMenu, DestroyMenu,
        TPM_LEFTALIGN, TPM_BOTTOMALIGN,
        GetCursorPos, AppendMenuA, MF_STRING
    };
    use windows_sys::Win32::Foundation::POINT;
    
    info!("Showing tray menu");
    
    unsafe {
        // Get cursor position
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point);
        
        // Create popup menu
        let menu = CreatePopupMenu();
        if menu != 0 {
            // Add items - using ASCII codes
            let settings = "Settings\0".as_ptr();
            let quit = "Exit\0".as_ptr();
            
            AppendMenuA(menu, MF_STRING, 1, settings);
            AppendMenuA(menu, MF_STRING, 2, quit);
            
            // Show menu
            let cmd = TrackPopupMenu(
                menu, 
                TPM_LEFTALIGN | TPM_BOTTOMALIGN, 
                point.x, point.y, 
                0, 0, std::ptr::null_mut()
            );
            
            // Process selection
            if cmd > 0 {
                // Map to our MenuAction enum
                let action = match cmd {
                    1 => MenuAction::OpenSettings,
                    2 => MenuAction::Quit,
                    _ => {
                        DestroyMenu(menu);
                        return;
                    },
                };
                
                // Forward to our action channel
                if let Some(menu_tx) = &*MENU_CHANNEL.lock() {
                    info!("Sending menu action: {:?}", action);
                    if let Err(e) = menu_tx.send(action) {
                        error!("Failed to send menu action: {}", e);
                    }
                }
            }
            
            // Clean up
            DestroyMenu(menu);
        }
    }
} 