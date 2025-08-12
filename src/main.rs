mod audio;
mod whisper;
mod ui;
mod input;
mod utils;
mod error;
mod model;
mod state;

use anyhow::{Result, Context};
use log::{error, info, warn, debug};
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use std::fs;
use std::path::Path;
use crossbeam_channel::{select, tick};
use simple_logger;

use crate::{
    audio::{list_audio_devices, headphone_keepalive_thread},
    whisper::load_model,
    ui::{update_tray_icon, process_menu_actions, cleanup_tray, AppState},
    input::start_keyboard_listener,
    utils::{acquire_instance_lock, parse_args, set_config, Args},
    state::get_state_update_receiver,
};

// Configuration constants
const LOCK_FILE_PATH: &str = "push-to-whisper.lock";

// Use the global EXIT_REQUESTED from utils for consistent shutdown handling
use crate::utils::EXIT_REQUESTED;

// Removed unused utility functions

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    simple_logger::init_with_level(log::Level::Debug).context("Failed to initialize logger")?;
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
    let enable_tray = !args.disable_tray;
    if enable_tray {
        if let Err(e) = ui::init_tray_icon() {
            warn!("Failed to initialize system tray: {}", e);
        } else {
            // Initial state update on main thread
            update_tray_icon(ui::AppState::Normal);
            info!("System tray icon initialized successfully");
        }
    }
    
    // Start headphone keepalive thread if enabled
    let config = utils::get_config();
    if config.headphone_keepalive_interval > 0 {
        info!("Starting headphone keepalive thread with interval of {}s", config.headphone_keepalive_interval);
        if let Err(e) = headphone_keepalive_thread(config.headphone_keepalive_interval) {
            warn!("Failed to start headphone keepalive thread: {}", e);
        }
    }
    
    // Start keyboard event listener
    let _keyboard_thread = thread::spawn(move || {
        if let Err(e) = start_keyboard_listener() {
            error!("Keyboard listener error: {}", e);
        }
    });
    
    // Main event loop
    let ticker = tick(Duration::from_millis(50)); // Balanced timing for responsiveness without excessive CPU usage
    let state_update_rx = get_state_update_receiver();
    
    let mut last_known_state = AppState::Normal; // Initialize state correctly
    
    loop {
        select! {
            // Listen for periodic ticks
            recv(ticker) -> _ => {
                // Check if exit was requested from outside (e.g., Ctrl+C)
                if EXIT_REQUESTED.load(Ordering::SeqCst) {
                    info!("External exit requested, shutting down...");
                    break;
                }
                
                // Process any menu actions from the system tray (T key press)
                if enable_tray {
                    if let Ok(exit_requested_from_menu) = process_menu_actions() {
                        if exit_requested_from_menu {
                            info!("Exit requested via menu, shutting down...");
                            EXIT_REQUESTED.store(true, Ordering::SeqCst);
                            // Let the loop break on the next check
                        }
                    }
                }
            },
            // Listen for state updates from other threads
            recv(state_update_rx) -> msg => {
                match msg {
                    Ok(new_state) => {
                        debug!("[Main Thread] Received state update: {:?}", new_state);
                        // Only update if the state has actually changed
                        if new_state != last_known_state {
                            info!("[Main Thread] App state changed from {:?} to {:?}", last_known_state, new_state);
                            if enable_tray {
                                // Call update_tray_icon from the main thread
                                update_tray_icon(new_state);
                            }
                            last_known_state = new_state;
                        } else {
                            debug!("[Main Thread] State {:?} is same as last known state, skipping UI update.", new_state);
                        }
                    }
                    Err(e) => {
                        error!("State update channel error: {}. Shutting down.", e);
                        EXIT_REQUESTED.store(true, Ordering::SeqCst);
                        break; // Exit loop on channel error
                    }
                }
            }
        }
        
        // Check exit condition again after select!
        if EXIT_REQUESTED.load(Ordering::SeqCst) {
             info!("Exit condition met after select!, breaking loop...");
             break;
        }
    }
    
    // Cleanup resources before exit
    if enable_tray {
        cleanup_tray();
    }
    
    // Clean up lock file on exit
    if let Err(e) = fs::remove_file(LOCK_FILE_PATH) {
        error!("Failed to remove lock file on exit: {}", e);
    }
    
    info!("Application shutdown complete");
    Ok(())
}

async fn init_app(args: &Args) -> Result<()> {
    // Store the configuration globally so other modules can access it
    set_config(args);
    
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
    })?;
    
    // List available audio devices for troubleshooting
    if let Err(e) = list_audio_devices() {
        warn!("Failed to list audio devices: {}", e);
    }
    
    // Load the whisper model with the specified model size
    load_model(&args.model_size).await?;
    
    Ok(())
}
