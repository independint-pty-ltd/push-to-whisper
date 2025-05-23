use anyhow::{anyhow, Result};
use log::{error, info};
use once_cell::sync::OnceCell;
use std::fs::{File, OpenOptions};
use std::sync::atomic::AtomicBool;
use std::process::Command;
use std::path::Path;
use std::io::Read;
use std::fs;


// Configuration
pub const LOCK_FILE_PATH: &str = "push-to-whisper.lock";
pub const CONFIG_FILE_PATH: &str = "push-to-whisper.config";

#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub disable_beep: bool,
    pub disable_tray: bool,
    pub disable_visual: bool,
    pub model_size: String,
    pub long_press_threshold: u64,
    pub headphone_keepalive_interval: u64,
    pub enable_debug_recording: bool,
    pub force_cpu: bool,
    pub beep_volume: f32,
}

// Global state
pub static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);
static IGNORE_EXIT_UNTIL: AtomicBool = AtomicBool::new(false);
static INSTANCE_LOCK: OnceCell<File> = OnceCell::new();
static CONFIG: once_cell::sync::Lazy<std::sync::Mutex<Option<Args>>> = 
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(None));

// Valid model sizes
pub const VALID_MODELS: [&str; 5] = ["tiny.en", "base.en", "small.en", "medium.en", "large"];
pub const DEFAULT_MODEL: &str = "medium.en";
pub const DEFAULT_LONG_PRESS_THRESHOLD: u64 = 150; // milliseconds
pub const DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL: u64 = 30; // seconds
pub const DEFAULT_ENABLE_DEBUG_RECORDING: bool = true; // enabled by default for troubleshooting
pub const DEFAULT_BEEP_VOLUME: f32 = 0.3; // 30% volume by default

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

fn merge_config_with_defaults(existing_config: &str) -> String {
    let mut config_lines: Vec<String> = existing_config.lines().map(String::from).collect();
    let mut has_changes = false;
    
    // Check for missing configurations and add them with defaults
    let required_configs = [
        ("enable_beep", "true", "Audio feedback (true/false)"),
        ("enable_tray", "true", "System tray icon (true/false)"),
        ("enable_visual", "true", "Visual feedback (true/false)"),
        ("model_size", DEFAULT_MODEL, "Whisper model size (tiny.en, base.en, small.en, medium.en, large)"),
        ("long_press_threshold", &DEFAULT_LONG_PRESS_THRESHOLD.to_string(), "Long press threshold in milliseconds"),
        ("headphone_keepalive_interval", &DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL.to_string(), "Headphone keepalive interval in seconds"),
        ("enable_debug_recording", &DEFAULT_ENABLE_DEBUG_RECORDING.to_string(), "Debug recording (true/false)"),
        ("force_cpu", "false", "Force CPU mode (true/false)"),
        ("beep_volume", &DEFAULT_BEEP_VOLUME.to_string(), "Beep volume (0.0 to 1.0)"),
    ];
    
    for (key, default_value, comment) in required_configs {
        if !config_lines.iter().any(|line| {
            let line = line.trim();
            !line.starts_with('#') && line.split('=').next().map_or(false, |k| k.trim() == key)
        }) {
            // Add a blank line before new sections if there isn't one
            if !config_lines.last().map_or(true, |line| line.trim().is_empty()) {
                config_lines.push(String::new());
            }
            // Add the comment and setting as owned strings
            config_lines.push(format!("# {}", comment));
            config_lines.push(format!("{} = {}", key, default_value));
            has_changes = true;
        }
    }
    
    if has_changes {
        config_lines.join("\n") + "\n"
    } else {
        existing_config.to_string()
    }
}

fn create_default_config_if_not_exists() -> Result<()> {
    let config_path = Path::new(CONFIG_FILE_PATH);
    
    if config_path.exists() {
        // Read existing config
        let existing_config = fs::read_to_string(config_path)?;
        
        // Merge with defaults
        let merged_config = merge_config_with_defaults(&existing_config);
        
        // Only write if changes were made
        if merged_config != existing_config {
            info!("Updating configuration file with new default settings");
            fs::write(config_path, merged_config)?;
        }
    } else {
        info!("Creating default configuration file at {}", CONFIG_FILE_PATH);
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
            # Debug recording (true/false)\n\
            # Saves audio to debug_recording.wav for troubleshooting\n\
            enable_debug_recording = {}\n\
            \n\
            # Force CPU mode (true/false)\n\
            # Set to true to disable GPU acceleration and use CPU only\n\
            force_cpu = false\n\
            \n\
            # Beep volume (0.0 to 1.0)\n\
            beep_volume = {}\n",
            DEFAULT_MODEL,
            DEFAULT_LONG_PRESS_THRESHOLD,
            DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL,
            DEFAULT_ENABLE_DEBUG_RECORDING,
            DEFAULT_BEEP_VOLUME
        );
        
        fs::write(config_path, config_content)?;
    }
    
    Ok(())
}

fn read_config_file() -> Args {
    // Create default config if it doesn't exist
    if let Err(e) = create_default_config_if_not_exists() {
        error!("Failed to create default config file: {}", e);
    }
    
    // Default values
    let mut enable_beep = true;
    let mut enable_tray = true;
    let mut enable_visual = true;
    let mut model_size = DEFAULT_MODEL.to_string();
    let mut long_press_threshold = DEFAULT_LONG_PRESS_THRESHOLD;
    let mut headphone_keepalive_interval = DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL;
    let mut enable_debug_recording = DEFAULT_ENABLE_DEBUG_RECORDING;
    let mut force_cpu = false;
    let mut beep_volume = DEFAULT_BEEP_VOLUME;
    
    // Try to read config file
    if let Ok(mut file) = File::open(CONFIG_FILE_PATH) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
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
                            enable_beep = value.to_lowercase() == "true";
                        },
                        "enable_tray" => {
                            enable_tray = value.to_lowercase() == "true";
                        },
                        "enable_visual" => {
                            enable_visual = value.to_lowercase() == "true";
                        },
                        "model_size" => {
                            if VALID_MODELS.contains(&value) {
                                model_size = value.to_string();
                            } else {
                                error!("Invalid model size in config: {}", value);
                            }
                        },
                        "long_press_threshold" => {
                            if let Ok(val) = value.parse::<u64>() {
                                long_press_threshold = val;
                            }
                        },
                        "headphone_keepalive_interval" => {
                            if let Ok(val) = value.parse::<u64>() {
                                headphone_keepalive_interval = val;
                            }
                        },
                        "enable_debug_recording" => {
                            enable_debug_recording = value.to_lowercase() == "true";
                        },
                        "force_cpu" => {
                            force_cpu = value.to_lowercase() == "true";
                        },
                        "beep_volume" => {
                            if let Ok(val) = value.parse::<f32>() {
                                beep_volume = val;
                            }
                        },
                        _ => {
                            // Unknown key, ignore
                        }
                    }
                }
            }
        }
    }
    
    Args {
        disable_beep: !enable_beep,
        disable_tray: !enable_tray,
        disable_visual: !enable_visual,
        model_size,
        long_press_threshold,
        headphone_keepalive_interval,
        enable_debug_recording,
        force_cpu,
        beep_volume,
    }
}

pub fn parse_args() -> Args {
    // First read from config file
    let mut args = read_config_file();
    
    // Then override with command line arguments
    let mut i = 1;
    while i < std::env::args().len() {
        let arg = std::env::args().nth(i).unwrap();
        
        match arg.as_str() {
            "--no-beep" => {
                args.disable_beep = true;
                i += 1;
            },
            "--no-tray" => {
                args.disable_tray = true;
                i += 1;
            },
            "--no-visual" => {
                args.disable_visual = true;
                i += 1;
            },
            "--model-size" | "-m" => {
                if let Some(value) = std::env::args().nth(i + 1) {
                    if VALID_MODELS.contains(&value.as_str()) {
                        args.model_size = value;
                    } else {
                        error!("Invalid model size: {}", value);
                        error!("Valid models: {:?}", VALID_MODELS);
                    }
                    i += 2;
                } else {
                    error!("Missing value for --model-size");
                    i += 1;
                }
            },
            "--long-press-threshold" | "--lpt" => {
                if let Some(value) = std::env::args().nth(i + 1) {
                    if let Ok(val) = value.parse::<u64>() {
                        args.long_press_threshold = val;
                    } else {
                        error!("Invalid value for long press threshold: {}", value);
                    }
                    i += 2;
                } else {
                    error!("Missing value for --long-press-threshold");
                    i += 1;
                }
            },
            "--headphone-keepalive" | "--hk" => {
                if let Some(value) = std::env::args().nth(i + 1) {
                    if let Ok(val) = value.parse::<u64>() {
                        args.headphone_keepalive_interval = val;
                    } else {
                        error!("Invalid value for headphone keepalive interval: {}", value);
                    }
                    i += 2;
                } else {
                    error!("Missing value for --headphone-keepalive");
                    i += 1;
                }
            },
            "--debug-recording" => {
                args.enable_debug_recording = true;
                i += 1;
            },
            "--no-debug-recording" => {
                args.enable_debug_recording = false;
                i += 1;
            },
            "--force-cpu" | "--no-gpu" => {
                args.force_cpu = true;
                i += 1;
            },
            "--beep-volume" => {
                if let Some(value) = std::env::args().nth(i + 1) {
                    if let Ok(val) = value.parse::<f32>() {
                        if val >= 0.0 && val <= 1.0 {
                            args.beep_volume = val;
                        } else {
                            error!("Beep volume must be between 0.0 and 1.0");
                        }
                    } else {
                        error!("Invalid value for beep volume: {}", value);
                    }
                    i += 2;
                } else {
                    error!("Missing value for --beep-volume");
                    i += 1;
                }
            },
            _ => {
                // Unknown argument, ignore
                i += 1;
            }
        }
    }
    
    args
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
            force_cpu: false,
            beep_volume: DEFAULT_BEEP_VOLUME,
        }
    }
}

/// Save the provided configuration to the config file
pub fn save_config(args: &Args) -> Result<()> {
    let config_path = Path::new(CONFIG_FILE_PATH);
    
    // Create a string representation of the config
    let config_content = format!(
        "# Push-to-Whisper Configuration File\n\
        # Edit this file to change default settings\n\
        # Command line arguments will override these settings\n\
        \n\
        # Audio feedback (true/false)\n\
        enable_beep = {}\n\
        \n\
        # System tray icon (true/false)\n\
        enable_tray = {}\n\
        \n\
        # Visual feedback (true/false)\n\
        enable_visual = {}\n\
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
        # Debug recording (true/false)\n\
        # Saves audio to debug_recording.wav for troubleshooting\n\
        enable_debug_recording = {}\n\
        \n\
        # Force CPU mode (true/false)\n\
        # Set to true to disable GPU acceleration and use CPU only\n\
        force_cpu = {}\n\
        \n\
        # Beep volume (0.0 to 1.0)\n\
        beep_volume = {}\n",
        !args.disable_beep,
        !args.disable_tray,
        !args.disable_visual,
        args.model_size,
        args.long_press_threshold,
        args.headphone_keepalive_interval,
        args.enable_debug_recording,
        args.force_cpu,
        args.beep_volume
    );
    
    // Write the config to the file
    fs::write(config_path, config_content)?;
    
    // Update the global config
    *CONFIG.lock().unwrap() = Some(args.clone());
    
    Ok(())
} 