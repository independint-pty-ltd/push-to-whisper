mod audio;
mod whisper;
mod ui;
mod input;
mod utils;
mod error;
mod model;

use anyhow::{Result, Context};
use log::{error, info, warn};
use rdev::Event;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use crossbeam_channel::{bounded, Sender, Receiver, select, tick};
use simple_logger;

use crate::{
    audio::{start_recording, stop_recording, play_beep_blocking},
    whisper::{transcribe_audio, load_model},
    ui::update_tray_icon,
    input::{handle_keyboard_event, start_keyboard_listener},
    utils::{acquire_instance_lock, parse_args, request_exit, Args},
};

// Configuration constants
const HOTKEY: u32 = 0x51; // 'Q' key
const LOCK_FILE_PATH: &str = "push-to-whisper.lock";
const ENABLE_SYSTEM_TRAY: bool = true;
const ENABLE_BEEP_SOUNDS: bool = true;

// Global state
static RECORDING: AtomicBool = AtomicBool::new(false);
static LAST_ACTIVITY_TIME: AtomicU64 = AtomicU64::new(0);
static LAST_ESC_PRESS: AtomicU64 = AtomicU64::new(0);
static HOTKEY_PRESS_TIME: AtomicU64 = AtomicU64::new(0);
static HOTKEY_DOWN: AtomicBool = AtomicBool::new(false);
static IGNORE_EXIT_UNTIL: AtomicU64 = AtomicU64::new(0);
static KEY_HANDLED: AtomicBool = AtomicBool::new(false);
static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);

// Update the last activity timestamp
fn update_activity_time() {
    let now = get_current_time_ms();
    LAST_ACTIVITY_TIME.store(now, Ordering::SeqCst);
}

fn get_current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    simple_logger::init_with_level(log::Level::Info).context("Failed to initialize logger")?;
    info!("Starting Push-to-Whisper...");
    
    // Parse command line arguments
    let args = parse_args();
    
    // Check for other running instances
    if let Err(e) = acquire_instance_lock() {
        error!("Another instance is already running: {}", e);
        return Err(anyhow::anyhow!("Another instance is already running"));
    }
    
    // Initialize the application
    init_app(&args).await.context("Failed to initialize application")?;
    
    // Initialize the system tray
    if !args.disable_tray {
        update_tray_icon(false);
        info!("System tray icon initialized successfully");
    }
    
    // Start keyboard event listener
    let keyboard_thread = thread::spawn(move || {
        if let Err(e) = start_keyboard_listener() {
            error!("Keyboard listener error: {}", e);
        }
    });
    
    // Start headphone keepalive thread if enabled
    let mut headphone_thread = None;
    if args.headphone_keepalive_interval > 0 {
        info!("Starting headphone keepalive thread with interval of {}s", args.headphone_keepalive_interval);
        let interval = args.headphone_keepalive_interval;
        headphone_thread = Some(thread::spawn(move || {
            let interval_duration = Duration::from_secs(interval);
            loop {
                if EXIT_REQUESTED.load(Ordering::SeqCst) {
                    break;
                }
                
                // Only play keepalive beep if we're recording
                if RECORDING.load(Ordering::SeqCst) {
                    if let Err(e) = play_beep_blocking(1000, 100) {
                        error!("Failed to play keepalive beep: {}", e);
                    }
                }
                
                thread::sleep(interval_duration);
            }
        }));
    }
    
    // Main event loop
    let ticker = tick(Duration::from_millis(100));
    loop {
        select! {
            recv(ticker) -> _ => {
                // Check if exit was requested
                if EXIT_REQUESTED.load(Ordering::SeqCst) {
                    info!("Exit requested, shutting down...");
                    break;
                }
                
                // Update tray icon based on recording state
                if !args.disable_tray {
                    update_tray_icon(RECORDING.load(Ordering::SeqCst));
                }
            }
        }
    }
    
    info!("Application shutdown complete");
    Ok(())
}

async fn init_app(args: &Args) -> Result<()> {
    // The lock file handling is now done in acquire_instance_lock()
    // which is called before this function
    
    // Create lock file - this should already be done by acquire_instance_lock
    // but we'll ensure it exists here as well
    if !Path::new(LOCK_FILE_PATH).exists() {
        fs::write(LOCK_FILE_PATH, "locked")?;
    }
    
    // Set up cleanup on exit
    ctrlc::set_handler(|| {
        EXIT_REQUESTED.store(true, Ordering::SeqCst);
        // Clean up lock file on exit
        if let Err(e) = fs::remove_file(LOCK_FILE_PATH) {
            error!("Failed to remove lock file on exit: {}", e);
        }
    })?;
    
    // Load the whisper model with the specified model size
    load_model(&args.model_size).await?;
    
    Ok(())
}
