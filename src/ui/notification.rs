use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use log::{debug, error, info};
use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::ui::AppState;

// Global state for notification management
static CURRENT_NOTIFICATION_THREAD: Lazy<Mutex<Option<NotificationThread>>> = Lazy::new(|| Mutex::new(None));
static NOTIFICATION_SYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Persistent notification using a combination of Toast + Console logging
struct NotificationThread {
    state: AppState,
    should_stop: std::sync::Arc<AtomicBool>,
    _handle: std::thread::JoinHandle<()>,
}

impl NotificationThread {
    fn new(state: AppState) -> Result<Self, Box<dyn std::error::Error>> {
        let message = match state {
            AppState::Recording => "🔴 RECORDING - Push-to-Whisper is listening for audio...",
            AppState::Transcribing => "🟠 TRANSCRIBING - Push-to-Whisper is processing audio...",
            AppState::Normal => return Err("No thread for Normal state".into()),
        };

        let should_stop = std::sync::Arc::new(AtomicBool::new(false));
        let should_stop_clone = should_stop.clone();
        let message = message.to_string();
        let state_clone = state;

        let handle = thread::spawn(move || {
            info!("🔔 PERSISTENT NOTIFICATION: {}", message);
            
            // Show initial toast notification
            #[cfg(windows)]
            {
                use winrt_notification::{Toast, Duration as ToastDuration};
                let (title, body) = match state_clone {
                    AppState::Recording => ("🔴 RECORDING", "Push-to-Whisper is listening..."),
                    AppState::Transcribing => ("🟠 TRANSCRIBING", "Push-to-Whisper is processing..."),
                    AppState::Normal => unreachable!(),
                };
                
                if let Err(e) = Toast::new(Toast::POWERSHELL_APP_ID)
                    .title(title)
                    .text1(body)
                    .duration(ToastDuration::Long)
                    .show()
                {
                    debug!("Failed to show toast: {}", e);
                }
            }

            let start_time = Instant::now();
            let mut last_log_time = Instant::now();

            // Keep the notification alive with periodic logging
            while !should_stop_clone.load(Ordering::SeqCst) {
                // Log status every 10 seconds to show persistence
                if last_log_time.elapsed() >= Duration::from_secs(10) {
                    let elapsed = start_time.elapsed().as_secs();
                    info!("🔔 {} ({}s elapsed)", message, elapsed);
                    last_log_time = Instant::now();
                }

                thread::sleep(Duration::from_millis(500));
            }

            info!("🔔 NOTIFICATION ENDED: {}", message);
        });

        Ok(NotificationThread {
            state,
            should_stop,
            _handle: handle,
        })
    }

    fn close(self) {
        info!("Closing persistent notification for state: {:?}", self.state);
        self.should_stop.store(true, Ordering::SeqCst);
        // Thread will exit and be cleaned up automatically
    }
}



/// Initialize the notification system
pub fn init_notification_system() {
    if NOTIFICATION_SYSTEM_INITIALIZED.load(Ordering::SeqCst) {
        return; // Already initialized
    }
    
    NOTIFICATION_SYSTEM_INITIALIZED.store(true, Ordering::SeqCst);
    info!("Custom notification window system initialized");
}

/// Show a notification for the given state
pub fn show_notification(state: AppState) {
    // Check if visual feedback is disabled
    let config = crate::utils::get_config();
    if config.disable_visual {
        debug!("Visual feedback is disabled in config, skipping GUI notification");
        return;
    }

    // Initialize notification system if not already done
    if !NOTIFICATION_SYSTEM_INITIALIZED.load(Ordering::SeqCst) {
        init_notification_system();
    }

    // Handle the notification in current thread to avoid thread-safety issues
    let mut current_thread = CURRENT_NOTIFICATION_THREAD.lock();
    
    // Close any existing notification thread
    if let Some(thread) = current_thread.take() {
        thread.close();
        info!("Closed previous notification thread");
    }
    
    match state {
        AppState::Normal => {
            // Just close any existing notification (already done above)
            info!("Closed notification thread (Normal state)");
        }
        AppState::Recording | AppState::Transcribing => {
            // Create new persistent notification thread
            match NotificationThread::new(state) {
                Ok(thread) => {
                    info!("Created persistent notification thread for state: {:?}", state);
                    *current_thread = Some(thread);
                }
                Err(e) => {
                    error!("Failed to create persistent notification thread: {}", e);
                    // Fallback to console notification
                    let message = match state {
                        AppState::Recording => "🔴 RECORDING - Push-to-Whisper is listening for audio...",
                        AppState::Transcribing => "🟠 TRANSCRIBING - Push-to-Whisper is processing audio...",
                        AppState::Normal => unreachable!(),
                    };
                    info!("🔔 NOTIFICATION: {}", message);
                }
            }
        }
    }
} 