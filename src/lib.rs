// Re-export modules
pub mod ui;
pub mod audio;
pub mod whisper;
pub mod input;
pub mod utils;
pub mod error;
pub mod model;

// Re-export commonly used items
pub use error::AppError;
pub use audio::AudioConfig;
pub use whisper::WhisperConfig;
pub use ui::TrayConfig;
pub use input::InputConfig;

// Re-export AppState from ui module
pub use crate::ui::AppState; 