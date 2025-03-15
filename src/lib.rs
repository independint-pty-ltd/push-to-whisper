pub mod audio;
pub mod error;
pub mod input;
pub mod model;
pub mod ui;
pub mod utils;
pub mod whisper;

// Re-export commonly used items
pub use error::AppError;
pub use audio::AudioConfig;
pub use whisper::WhisperConfig;
pub use ui::TrayConfig;
pub use input::InputConfig; 