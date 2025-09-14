use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use enigo::{Enigo, Settings, Keyboard};
#[cfg(not(target_os = "windows"))]
use enigo::{Key, Direction};
use log::{error, info, warn};
use rdev::{Event, EventType, Key as RdevKey};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
#[cfg(target_os = "windows")]
use std::sync::atomic::AtomicIsize;
use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};
use std::thread;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_V};
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};
#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
static LAST_FG_HWND: AtomicIsize = AtomicIsize::new(0);

// use crate::error::AppError; // Currently unused
use crate::state::send_state_update;
use crate::state::RECORDING;
use crate::ui::AppState;
use crate::utils::EXIT_REQUESTED;

// Configuration
pub const HOTKEY: RdevKey = RdevKey::AltGr; // Default to Right Alt (AltGr); overridden by runtime config
pub const LONG_PRESS_THRESHOLD: u64 = 500; // Reduced from 1000ms to 500ms for easier triggering
pub const TEXT_INSERT_METHOD: TextInsertMethod = TextInsertMethod::Clipboard;
pub const CLIPBOARD_RESTORE_DELAY: std::time::Duration = std::time::Duration::from_secs(10);

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)] // Configuration enum - variants may be used in future
pub enum TextInsertMethod {
    Clipboard,
    Shortcut,
    Typing,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Configuration struct - fields may be used in future
pub struct InputConfig {
    pub hotkey: RdevKey,
    pub long_press_threshold: u64,
    pub text_insert_method: TextInsertMethod,
    pub clipboard_restore_delay: std::time::Duration,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            hotkey: HOTKEY,
            long_press_threshold: LONG_PRESS_THRESHOLD,
            text_insert_method: TEXT_INSERT_METHOD,
            clipboard_restore_delay: CLIPBOARD_RESTORE_DELAY,
        }
    }
}

// Global state
static LAST_ACTIVITY_TIME: AtomicU64 = AtomicU64::new(0);
static LAST_ESC_PRESS: AtomicU64 = AtomicU64::new(0);
static HOTKEY_PRESS_TIME: AtomicU64 = AtomicU64::new(0);
static HOTKEY_DOWN: AtomicBool = AtomicBool::new(false);
static KEY_HANDLED: AtomicBool = AtomicBool::new(false);
static IGNORE_EXIT_UNTIL: AtomicU64 = AtomicU64::new(0);

fn get_current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn update_activity_time() {
    LAST_ACTIVITY_TIME.store(get_current_time_ms(), Ordering::SeqCst);
}

pub fn handle_keyboard_event(event: Event) -> Result<()> {
    let now_ms = Instant::now().elapsed().as_millis() as u64;
    
    match event.event_type {
        EventType::KeyPress(key) => {
            // Determine configured hotkey at runtime
            let configured_hotkey = match crate::utils::get_config().hotkey.as_str() {
                "right_ctrl" => RdevKey::ControlRight,
                _ => RdevKey::AltGr,
            };

            if key == configured_hotkey && !KEY_HANDLED.load(Ordering::SeqCst) {
                KEY_HANDLED.store(true, Ordering::SeqCst);
                // Remember current foreground window to restore focus later
                #[cfg(target_os = "windows")]
                {
                    let hwnd = unsafe { GetForegroundWindow() };
                    let hwnd_val = hwnd as isize;
                    if hwnd_val != 0 {
                        LAST_FG_HWND.store(hwnd_val, Ordering::SeqCst);
                    }
                }
                
                if !RECORDING.load(Ordering::SeqCst) && !HOTKEY_DOWN.load(Ordering::SeqCst) {
                    info!("Hotkey pressed - waiting for long press threshold");
                    HOTKEY_PRESS_TIME.store(now_ms, Ordering::SeqCst);
                    HOTKEY_DOWN.store(true, Ordering::SeqCst);
                    
                    let threshold = crate::utils::get_config().long_press_threshold;
                    
                    thread::spawn(move || {
                        thread::sleep(std::time::Duration::from_millis(threshold));
                        
                        if HOTKEY_DOWN.load(Ordering::SeqCst) && !RECORDING.load(Ordering::SeqCst) {
                            info!("Long press threshold reached - starting recording");
                            if let Err(e) = crate::audio::start_recording() {
                                error!("Failed to start recording: {}", e);
                            } else {
                                // Successfully started recording, update state via channel
                                RECORDING.store(true, Ordering::SeqCst);
                                send_state_update(AppState::Recording);
                            }
                        } else {
                            if !HOTKEY_DOWN.load(Ordering::SeqCst) {
                                info!("Key released before threshold - not recording");
                            }
                            if RECORDING.load(Ordering::SeqCst) {
                                info!("Already recording - ignoring key press");
                            }
                        }
                    });
                }
            } 
            else if key == RdevKey::Escape {
                let now = get_current_time_ms();
                let last_press = LAST_ESC_PRESS.load(Ordering::SeqCst);
                LAST_ESC_PRESS.store(now, Ordering::SeqCst);

                // Check for double-press within 500ms
                if now - last_press < 500 {
                    info!("Double ESC pressed, exiting...");
                    EXIT_REQUESTED.store(true, Ordering::SeqCst);
                }
            }
        },
        EventType::KeyRelease(key) => {
            // Determine configured hotkey at runtime
            let configured_hotkey = match crate::utils::get_config().hotkey.as_str() {
                "right_ctrl" => RdevKey::ControlRight,
                _ => RdevKey::AltGr,
            };
            if key == configured_hotkey {
                info!("Hotkey released");
                KEY_HANDLED.store(false, Ordering::SeqCst);
                HOTKEY_DOWN.store(false, Ordering::SeqCst);
                
                if RECORDING.load(Ordering::SeqCst) {
                    info!("Stopping recording and starting transcription...");
                    // Don't update state yet - let the audio module handle it
                    
                    // First stop the audio recording while RECORDING flag is still true
                    if let Err(e) = crate::audio::stop_recording() {
                        error!("Failed to stop recording: {}", e);
                        // If stopping failed, maybe revert state?
                        send_state_update(AppState::Normal); // Revert to Normal if stop fails
                    }
                    
                    // Now we can set the recording flag to false
                    // This is needed to prevent duplicate stop_recording calls if the user
                    // presses the key again quickly
                    RECORDING.store(false, Ordering::SeqCst);
                    
                    // The audio::stop_recording function will handle the state transitions:
                    // 1. AppState::Transcribing when starting transcription
                    // 2. AppState::Normal when transcription completes
                } else {
                    info!("Hotkey released but wasn't recording.");
                }
            }
        },
        _ => {} 
    }
    
    Ok(())
}

pub fn type_text(text: &str) -> Result<()> {
    if text.trim().is_empty() {
        warn!("Empty text, nothing to type");
        return Ok(());
    }
    
    info!("Inserting text: '{}'", text);
    
    // Add visual feedback for processing (but no beep)
    info!("📝 PROCESSING TEXT 📝");
    
    // Temporarily disable exit
    let now = Instant::now().elapsed().as_secs();
    IGNORE_EXIT_UNTIL.store(now + 15, Ordering::SeqCst);
    
    match TEXT_INSERT_METHOD {
        TextInsertMethod::Clipboard => {
            // Use a concrete type (ClipboardContext) instead of trait inference
            match ClipboardContext::new() {
                Ok(mut ctx) => {
                    if let Err(e) = ctx.set_contents(text.to_string()) {
                        warn!("Failed to set clipboard contents: {:?}", e);
                        return Ok(());
                    }
                    
                    info!("Text copied to clipboard");
                    
                    // Give the target app a brief moment before pasting
                    #[cfg(target_os = "windows")]
                    {
                        let hwnd_val = LAST_FG_HWND.load(Ordering::SeqCst);
                        if hwnd_val != 0 {
                            unsafe { SetForegroundWindow(hwnd_val as HWND); }
                        }
                    }
                    thread::sleep(Duration::from_millis(50));
                    simulate_ctrl_v();
                    info!("Paste attempted with keyboard shortcut");
                },
                Err(e) => {
                    warn!("Failed to access clipboard: {:?}", e);
                    return Ok(());
                }
            }
        },
        TextInsertMethod::Shortcut => {
            // Just use keyboard shortcut after setting clipboard
            match ClipboardContext::new() {
                Ok(mut ctx) => {
                    if let Err(e) = ctx.set_contents(text.to_string()) {
                        warn!("Failed to set clipboard contents: {:?}", e);
                        return Ok(());
                    }
                    
                    #[cfg(target_os = "windows")]
                    {
                        let hwnd_val = LAST_FG_HWND.load(Ordering::SeqCst);
                        if hwnd_val != 0 {
                            unsafe { SetForegroundWindow(hwnd_val as HWND); }
                        }
                    }
                    thread::sleep(Duration::from_millis(50));
                    simulate_ctrl_v();
                },
                Err(e) => {
                    warn!("Failed to access clipboard: {:?}", e);
                    return Ok(());
                }
            }
        },
        TextInsertMethod::Typing => {
            // Type each character individually
            let settings = Settings::default();
            let mut enigo = Enigo::new(&settings).unwrap_or_else(|_| Enigo::new(&Settings::default()).expect("Failed to init Enigo"));
            for c in text.chars() {
                let _ = enigo.text(&c.to_string());
                thread::sleep(Duration::from_millis(1)); // Reduced from 2ms to 1ms for faster typing
            }
        }
    }
    
    // Add visual feedback for completion (but no beep)
    info!("✅ TEXT PROCESSED ✅");
    
    Ok(())
}

#[cfg(target_os = "windows")]
fn simulate_ctrl_v() {
    unsafe {
        let mut inputs: [INPUT; 4] = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT { wVk: VK_CONTROL as u16, wScan: 0, dwFlags: 0, time: 0, dwExtraInfo: 0 },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT { wVk: VK_V as u16, wScan: 0, dwFlags: 0, time: 0, dwExtraInfo: 0 },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT { wVk: VK_V as u16, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT { wVk: VK_CONTROL as u16, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 },
                },
            },
        ];

        let _ = SendInput(inputs.len() as u32, inputs.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(not(target_os = "windows"))]
fn simulate_ctrl_v() {
    let settings = Settings::default();
    if let Ok(mut enigo) = Enigo::new(&settings) {
        let _ = enigo.key(Key::Control, Direction::Press);
        let _ = enigo.text("v");
        let _ = enigo.key(Key::Control, Direction::Release);
    }
}

pub fn start_keyboard_listener() -> Result<()> {
    info!("Starting keyboard event listener");
    rdev::listen(|event| {
        if let Err(e) = handle_keyboard_event(event) {
            error!("Error handling keyboard event: {}", e);
        }
    })
    .map_err(|e| anyhow::anyhow!("Failed to listen to keyboard events: {:?}", e))
} 