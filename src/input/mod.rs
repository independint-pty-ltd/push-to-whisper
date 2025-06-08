use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use enigo::{Enigo, Key, KeyboardControllable};
use log::{error, info, warn};
use rdev::{Event, EventType, Key as RdevKey};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};
use std::thread;

use crate::utils::request_exit;
// use crate::error::AppError; // Currently unused
use crate::state::send_state_update;
use crate::state::RECORDING;
use crate::ui::AppState;

// Configuration
pub const HOTKEY: RdevKey = RdevKey::ControlRight; // Using Right Control key as hotkey
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
            if key == HOTKEY && !KEY_HANDLED.load(Ordering::SeqCst) {
                KEY_HANDLED.store(true, Ordering::SeqCst);
                
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
                    request_exit();
                }
            }
        },
        EventType::KeyRelease(key) => {
            if key == HOTKEY {
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
                    
                    // Simulate Ctrl+V to paste
                    let mut enigo = Enigo::default();
                    thread::sleep(Duration::from_millis(50));
                    enigo.key_down(Key::Control);
                    enigo.key_click(Key::Layout('v'));
                    enigo.key_up(Key::Control);
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
                    
                    let mut enigo = Enigo::default();
                    enigo.key_down(Key::Control);
                    enigo.key_click(Key::Layout('v'));
                    enigo.key_up(Key::Control);
                },
                Err(e) => {
                    warn!("Failed to access clipboard: {:?}", e);
                    return Ok(());
                }
            }
        },
        TextInsertMethod::Typing => {
            // Type each character individually
            let mut enigo = Enigo::default();
            for c in text.chars() {
                enigo.key_click(Key::Layout(c));
                thread::sleep(Duration::from_millis(2)); // Reduced from 5ms to 2ms for faster typing
            }
        }
    }
    
    // Add visual feedback for completion (but no beep)
    info!("✅ TEXT PROCESSED ✅");
    
    Ok(())
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