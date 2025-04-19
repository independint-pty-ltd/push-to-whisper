// Re-exports
pub use std::sync::atomic::{AtomicBool, Ordering};

// Import and re-export modules
pub mod audio;
pub mod whisper;
pub mod ui;
pub mod input;
pub mod utils;
pub mod error;
pub mod model;
pub mod state;

// Re-export state-related items for easy access
pub use state::RECORDING;
pub use state::send_state_update;
pub use state::get_state_update_receiver;
pub use state::is_transcribing;

// Remove the duplicated TRANSCRIBING variable since it's in audio module
// pub static TRANSCRIBING: AtomicBool = AtomicBool::new(false);

// Re-export commonly used items
pub use error::AppError;
pub use audio::AudioConfig;
pub use whisper::WhisperConfig;
pub use input::InputConfig; 