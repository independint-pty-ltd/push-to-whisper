mod audio;
mod whisper;
mod ui;
mod input;
mod utils;
mod error;
mod model;

use anyhow::{Result, Context};
use log::{error, info, warn};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::Path;
use crossbeam_channel::{select, tick};
use simple_logger;

use crate::{
    audio::{list_audio_devices, headphone_keepalive_thread},
    whisper::load_model,
    ui::update_tray_icon,
    input::start_keyboard_listener,
    utils::{acquire_instance_lock, parse_args, Args},
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
    
    // Display acceleration mode information
    if whisper::is_cuda_available() {
        if whisper::is_using_cpu_fallback() {
            warn!("CUDA is available but using CPU fallback due to initialization failure");
        } else {
            info!("Using GPU acceleration with CUDA");
        }
    } else {
        if args.force_cpu {
            info!("Using CPU mode (forced by configuration)");
        } else {
            info!("Using CPU mode (CUDA not available)");
        }
    }
    
    // Initialize the system tray
    if !args.disable_tray {
        update_tray_icon(false);
        info!("System tray icon initialized successfully");
    }
    
    // Start headphone keepalive thread if enabled
    let config = utils::get_config();
    if config.headphone_keepalive_interval > 0 {
        info!("Starting headphone keepalive thread with interval of {}s", config.headphone_keepalive_interval);
        if let Err(e) = headphone_keepalive_thread() {
            warn!("Failed to start headphone keepalive thread: {}", e);
        }
    }
    
    // Start keyboard event listener
    let keyboard_thread = thread::spawn(move || {
        if let Err(e) = start_keyboard_listener() {
            error!("Keyboard listener error: {}", e);
        }
    });
    
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
    
    // List available audio devices for troubleshooting
    if let Err(e) = list_audio_devices() {
        warn!("Failed to list audio devices: {}", e);
    }
    
    // Load the whisper model with the specified model size
    load_model(&args.model_size).await?;
    
    Ok(())
}
