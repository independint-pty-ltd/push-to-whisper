use anyhow::Result;
use log::info;
use parking_lot::Mutex;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use once_cell::sync::Lazy;

use crate::error::AppError;
use super::model::ensure_model_exists;
use crate::utils::DEFAULT_MODEL;

// Global state
static WHISPER_CONTEXT: Lazy<Mutex<Option<WhisperContext>>> = Lazy::new(|| Mutex::new(None));
static MODEL_SIZE_USED: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(DEFAULT_MODEL.to_string()));

#[derive(Debug, Clone)]
pub struct WhisperConfig {
    pub model_size: String,
    pub language: String,
    pub translate: bool,
}

impl Default for WhisperConfig {
    fn default() -> Self {
        Self {
            model_size: DEFAULT_MODEL.to_string(),
            language: "en".to_string(),
            translate: false,
        }
    }
}

pub async fn load_model(model_size: &str) -> Result<()> {
    let mut context = WHISPER_CONTEXT.lock();
    if context.is_some() {
        // If the model is already loaded, check if it's the same size
        let current_model = MODEL_SIZE_USED.lock();
        if current_model.as_str() == model_size {
            return Ok(());
        }
        
        // If a different model is requested, unload the current one
        info!("Switching from model {} to {}", current_model, model_size);
        *context = None;
    }

    info!("Loading Whisper model: {}...", model_size);
    
    // Ensure model exists
    let model_path = ensure_model_exists(model_size).await?;
    
    let ctx = WhisperContext::new_with_params(
        &model_path.to_string_lossy(),
        WhisperContextParameters::default()
    ).map_err(|e| AppError::Whisper(format!("Failed to load model: {}", e)))?;
    
    *context = Some(ctx);
    
    // Update the model size used
    let mut current_model = MODEL_SIZE_USED.lock();
    *current_model = model_size.to_string();
    
    info!("Whisper model {} loaded successfully", model_size);
    Ok(())
}

pub fn transcribe_audio(audio_data: &[f32]) -> Result<String> {
    let context = WHISPER_CONTEXT.lock();
    let ctx = context.as_ref()
        .ok_or_else(|| AppError::Whisper("Whisper model not loaded".to_string()))?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("en"));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_timestamps(false);

    let mut state = ctx.create_state()
        .map_err(|e| AppError::Whisper(format!("Failed to create state: {}", e)))?;

    state.full(params, audio_data)
        .map_err(|e| AppError::Whisper(format!("Failed to process audio: {}", e)))?;

    let num_segments = state.full_n_segments()
        .map_err(|e| AppError::Whisper(format!("Failed to get number of segments: {}", e)))?;

    let mut text = String::new();
    for i in 0..num_segments {
        if let Ok(segment) = state.full_get_segment_text(i) {
            text.push_str(&segment);
        }
    }

    Ok(text.trim().to_string())
} 