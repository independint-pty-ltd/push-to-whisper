use std::sync::atomic::AtomicBool;
use log::error;
use once_cell::sync::Lazy;
use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::ui::AppState;

// Global state
pub static RECORDING: AtomicBool = AtomicBool::new(false);

// Export a reference to the TRANSCRIBING variable from the audio module
#[allow(dead_code)] // Function for future debugging features
pub fn is_transcribing() -> bool {
    crate::audio::is_transcribing()
}

// Channel for state updates to the main thread
static STATE_UPDATE_CHANNEL: Lazy<(Sender<AppState>, Receiver<AppState>)> = Lazy::new(|| unbounded());

// Function to send state updates
pub fn send_state_update(state: AppState) {
    if let Err(e) = STATE_UPDATE_CHANNEL.0.send(state) {
        error!("Failed to send state update {:?}: {}", state, e);
    }
}

// Function to get the state update receiver
pub fn get_state_update_receiver() -> Receiver<AppState> {
    STATE_UPDATE_CHANNEL.1.clone()
} 