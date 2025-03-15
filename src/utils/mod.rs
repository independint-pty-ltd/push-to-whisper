use anyhow::{anyhow, Result};
use log::{error, info};
use once_cell::sync::OnceCell;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::sync::atomic::AtomicBool;
use std::process::Command;
use std::path::Path;
use std::io::Read;
use std::fs;

use crate::error::AppError;

// Configuration
pub const LOCK_FILE_PATH: &str = "push-to-whisper.lock";
pub const CONFIG_FILE_PATH: &str = "push-to-whisper.config";

#[derive(Debug, Clone)]
pub struct Args {
    pub disable_beep: bool,
    pub disable_tray: bool,
    pub disable_visual: bool,
    pub model_size: String,
    pub long_press_threshold: u64,
    pub headphone_keepalive_interval: u64,
    pub enable_debug_recording: bool,
}

// Global state
static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);
static IGNORE_EXIT_UNTIL: AtomicBool = AtomicBool::new(false);
static INSTANCE_LOCK: OnceCell<File> = OnceCell::new();
static CONFIG: once_cell::sync::Lazy<std::sync::Mutex<Option<Args>>> = 
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(None));

// Valid model sizes
pub const VALID_MODELS: [&str; 5] = ["tiny.en", "base.en", "small.en", "medium.en", "large"];
pub const DEFAULT_MODEL: &str = "medium.en";
pub const DEFAULT_LONG_PRESS_THRESHOLD: u64 = 500; // milliseconds
pub const DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL: u64 = 30; // seconds
pub const DEFAULT_ENABLE_DEBUG_RECORDING: bool = false; // disabled by default

fn is_another_instance_running() -> bool {
    // Get the current process ID
    let current_pid = std::process::id();
    
    // On Windows, use tasklist to check for other instances
    if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq push-to-whisper.exe", "/FO", "CSV"])
            .output() 
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();
            
            // Count the number of instances (excluding header line)
            let instance_count = lines.len() - 1;
            
            // If more than one instance is found, another instance is running
            return instance_count > 1;
        }
    } else {
        // For Unix-like systems, we could use ps and grep
        if let Ok(output) = Command::new("ps")
            .args(["-e", "-o", "pid,comm"])
            .output() 
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut count = 0;
            
            for line in output_str.lines() {
                if line.contains("push-to-whisper") {
                    count += 1;
                }
            }
            
            // If more than one instance is found, another instance is running
            return count > 1;
        }
    }
    
    // If we couldn't determine, assume no other instance is running
    false
}

pub fn acquire_instance_lock() -> Result<()> {
    info!("Checking for other running instances...");
    
    // First check if another process is actually running
    if is_another_instance_running() {
        error!("Another instance of Push to Whisper is already running");
        return Err(anyhow!("Another instance is already running"));
    }
    
    // If lock file exists but no other process is running, it's stale
    if std::path::Path::new(LOCK_FILE_PATH).exists() {
        info!("Found stale lock file, removing it");
        if let Err(e) = std::fs::remove_file(LOCK_FILE_PATH) {
            error!("Failed to remove stale lock file: {}", e);
            // Continue anyway, the next open operation will fail if there's a real problem
        }
    }
    
    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(LOCK_FILE_PATH)
    {
        Ok(file) => {
            if INSTANCE_LOCK.set(file).is_err() {
                return Err(anyhow!("Failed to store lock file handle"));
            }
            
            // Note: Ctrl-C handler is now set in init_app, not here
            
            info!("Instance lock acquired successfully");
            Ok(())
        },
        Err(e) => {
            error!("Failed to create lock file: {}", e);
            Err(e.into())
        }
    }
}

pub fn request_exit() {
    if !IGNORE_EXIT_UNTIL.load(std::sync::atomic::Ordering::SeqCst) {
        EXIT_REQUESTED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

fn create_default_config_if_not_exists() -> Result<()> {
    if !Path::new(CONFIG_FILE_PATH).exists() {
        info!("Creating default configuration file");
        let config_content = format!(
            "# Push-to-Whisper Configuration File\n\
             # Edit this file to change default settings\n\
             # Command line arguments will override these settings\n\
             \n\
             # Audio feedback (true/false)\n\
             enable_beep = true\n\
             \n\
             # System tray icon (true/false)\n\
             enable_tray = true\n\
             \n\
             # Visual feedback (true/false)\n\
             enable_visual = true\n\
             \n\
             # Whisper model size (tiny.en, base.en, small.en, medium.en, large)\n\
             model_size = {}\n\
             \n\
             # Long press threshold in milliseconds (how long to hold the key before recording starts)\n\
             long_press_threshold = {}\n\
             \n\
             # Headphone keepalive interval in seconds (prevents wireless headphones from disconnecting)\n\
             # Set to 0 to disable\n\
             headphone_keepalive_interval = {}\n\
             \n\
             # Save debug recording to file (true/false)\n\
             # Creates debug_recording.wav in the application directory\n\
             enable_debug_recording = {}\n",
            DEFAULT_MODEL,
            DEFAULT_LONG_PRESS_THRESHOLD,
            DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL,
            DEFAULT_ENABLE_DEBUG_RECORDING
        );
        
        fs::write(CONFIG_FILE_PATH, config_content)?;
        info!("Default configuration file created successfully");
    }
    
    Ok(())
}

fn read_config_file() -> Args {
    // Default values
    let mut config = Args {
        disable_beep: false,
        disable_tray: false,
        disable_visual: false,
        model_size: DEFAULT_MODEL.to_string(),
        long_press_threshold: DEFAULT_LONG_PRESS_THRESHOLD,
        headphone_keepalive_interval: DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL,
        enable_debug_recording: DEFAULT_ENABLE_DEBUG_RECORDING,
    };
    
    // Try to read config file
    if let Ok(mut file) = File::open(CONFIG_FILE_PATH) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            info!("Reading configuration from file");
            
            // Parse each line
            for line in contents.lines() {
                let line = line.trim();
                
                // Skip comments and empty lines
                if line.starts_with('#') || line.is_empty() {
                    continue;
                }
                
                // Parse key-value pairs
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim();
                    
                    match key {
                        "enable_beep" => {
                            config.disable_beep = value != "true";
                        },
                        "enable_tray" => {
                            config.disable_tray = value != "true";
                        },
                        "enable_visual" => {
                            config.disable_visual = value != "true";
                        },
                        "model_size" => {
                            if VALID_MODELS.contains(&value) {
                                config.model_size = value.to_string();
                            } else {
                                error!("Invalid model size in config file: {}", value);
                                error!("Using default model: {}", DEFAULT_MODEL);
                            }
                        },
                        "long_press_threshold" => {
                            if let Ok(threshold) = value.parse::<u64>() {
                                if threshold > 0 {
                                    config.long_press_threshold = threshold;
                                } else {
                                    error!("Invalid long press threshold: {}, must be > 0", threshold);
                                }
                            } else {
                                error!("Invalid long press threshold: {}, must be a number", value);
                            }
                        },
                        "headphone_keepalive_interval" => {
                            if let Ok(interval) = value.parse::<u64>() {
                                config.headphone_keepalive_interval = interval;
                            } else {
                                error!("Invalid headphone keepalive interval: {}, must be a number", value);
                            }
                        },
                        "enable_debug_recording" => {
                            config.enable_debug_recording = value == "true";
                        },
                        _ => {
                            // Unknown key, just log a warning
                            error!("Unknown configuration key: {}", key);
                        }
                    }
                }
            }
            
            info!("Configuration loaded from file");
        }
    }
    
    config
}

pub fn parse_args() -> Args {
    // First, ensure config file exists (create with defaults if not)
    if let Err(e) = create_default_config_if_not_exists() {
        error!("Failed to create default config file: {}", e);
    }
    
    // Read config from file
    let mut config = read_config_file();
    
    // Then parse command line args (which override config file)
    let args: Vec<String> = std::env::args().collect();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--no-beep" | "--quiet" | "-q" => config.disable_beep = true,
            "--no-tray" => config.disable_tray = true,
            "--no-visual" => config.disable_visual = true,
            "--debug-recording" => config.enable_debug_recording = true,
            "--no-debug-recording" => config.enable_debug_recording = false,
            "--model-size" | "-m" => {
                if i + 1 < args.len() {
                    let size = &args[i + 1];
                    if VALID_MODELS.contains(&size.as_str()) {
                        config.model_size = size.clone();
                        i += 1; // Skip the next argument since we've used it
                    } else {
                        eprintln!("Error: Invalid model size '{}'. Valid options are: {:?}", size, VALID_MODELS);
                        eprintln!("Using default model: {}", DEFAULT_MODEL);
                    }
                } else {
                    eprintln!("Error: --model-size requires a value");
                    eprintln!("Using default model: {}", DEFAULT_MODEL);
                }
            },
            "--long-press-threshold" | "--lpt" => {
                if i + 1 < args.len() {
                    if let Ok(threshold) = args[i + 1].parse::<u64>() {
                        if threshold > 0 {
                            config.long_press_threshold = threshold;
                            i += 1;
                        } else {
                            eprintln!("Error: Long press threshold must be > 0");
                        }
                    } else {
                        eprintln!("Error: Long press threshold must be a number");
                    }
                } else {
                    eprintln!("Error: --long-press-threshold requires a value");
                }
            },
            "--headphone-keepalive" | "--hk" => {
                if i + 1 < args.len() {
                    if let Ok(interval) = args[i + 1].parse::<u64>() {
                        config.headphone_keepalive_interval = interval;
                        i += 1;
                    } else {
                        eprintln!("Error: Headphone keepalive interval must be a number");
                    }
                } else {
                    eprintln!("Error: --headphone-keepalive requires a value");
                }
            },
            "--help" | "-h" => {
                println!("Push to Whisper - Speech to Text Tool");
                println!("Usage: push-to-whisper [OPTIONS]");
                println!("Options:");
                println!("  --no-beep, --quiet, -q    Disable beep sounds");
                println!("  --no-tray                 Disable system tray");
                println!("  --no-visual               Disable visual feedback");
                println!("  --debug-recording         Enable saving debug recording to file");
                println!("  --no-debug-recording      Disable saving debug recording to file");
                println!("  --model-size, -m <size>   Set the model size (tiny.en, base.en, small.en, medium.en, large)");
                println!("                            Default: {}", DEFAULT_MODEL);
                println!("  --long-press-threshold, --lpt <ms>  Set the long press threshold in milliseconds");
                println!("                            Default: {}", DEFAULT_LONG_PRESS_THRESHOLD);
                println!("  --headphone-keepalive, --hk <sec>   Set the headphone keepalive interval in seconds");
                println!("                            Default: {}, 0 to disable", DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL);
                println!("  --help, -h                Show this help message");
                println!("");
                println!("Configuration:");
                println!("  Settings can also be specified in the {} file", CONFIG_FILE_PATH);
                println!("  Command line arguments override settings in the config file");
                std::process::exit(0);
            }
            _ => {
                eprintln!("Warning: Unknown argument: {}", args[i]);
                eprintln!("Use --help to see available options");
            }
        }
        i += 1;
    }

    info!("Using model size: {}", config.model_size);
    info!("Long press threshold: {}ms", config.long_press_threshold);
    info!("Headphone keepalive interval: {}s", config.headphone_keepalive_interval);
    info!("Debug recording: {}", if config.enable_debug_recording { "enabled" } else { "disabled" });
    
    // Store the config for later access
    *CONFIG.lock().unwrap() = Some(config.clone());
    
    config
}

/// Get the current configuration
pub fn get_config() -> Args {
    if let Some(config) = &*CONFIG.lock().unwrap() {
        config.clone()
    } else {
        // If config hasn't been initialized yet, return defaults
        Args {
            disable_beep: false,
            disable_tray: false,
            disable_visual: false,
            model_size: DEFAULT_MODEL.to_string(),
            long_press_threshold: DEFAULT_LONG_PRESS_THRESHOLD,
            headphone_keepalive_interval: DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL,
            enable_debug_recording: DEFAULT_ENABLE_DEBUG_RECORDING,
        }
    }
} 