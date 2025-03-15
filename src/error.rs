use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Audio error: {0}")]
    Audio(String),
    
    #[error("Whisper error: {0}")]
    Whisper(String),
    
    #[error("Device error: {0}")]
    Device(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Application already running")]
    AlreadyRunning,
} 