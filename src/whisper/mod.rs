use anyhow::Result;
use log::{info, warn, error};
use parking_lot::Mutex;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::error::AppError;
use super::model::ensure_model_exists;
use crate::utils::{DEFAULT_MODEL, get_config};

// Global state
static WHISPER_CONTEXT: Lazy<Mutex<Option<WhisperContext>>> = Lazy::new(|| Mutex::new(None));
static MODEL_SIZE_USED: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(DEFAULT_MODEL.to_string()));
static USING_CPU_FALLBACK: AtomicBool = AtomicBool::new(false);
static CUDA_AVAILABLE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
#[allow(dead_code)] // Configuration struct - fields may be used in future
pub struct WhisperConfig {
    pub model_size: String,
    pub language: String,
    pub translate: bool,
    pub force_cpu: bool,
}

impl Default for WhisperConfig {
    fn default() -> Self {
        Self {
            model_size: DEFAULT_MODEL.to_string(),
            language: "en".to_string(),
            translate: false,
            force_cpu: false,
        }
    }
}

/// Check if CUDA is available and working
fn check_cuda_availability() -> bool {
    #[cfg(feature = "cuda")]
    {
        // First check if nvidia-smi is available
        if let Ok(output) = std::process::Command::new("nvidia-smi").output() {
            if output.status.success() {
                info!("NVIDIA GPU detected via nvidia-smi");
                return true;
            }
        }
        
        // Check for CUDA libraries
        #[cfg(target_os = "windows")]
        {
            use std::path::Path;
            
            // Check common CUDA library locations on Windows
            let cuda_paths = [
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.0\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.1\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.2\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.3\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.4\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.5\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.6\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.7\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v11.8\\bin\\cudart64_110.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.0\\bin\\cudart64_12.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.1\\bin\\cudart64_12.dll",
                "C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.2\\bin\\cudart64_12.dll",
            ];
            
            for path in cuda_paths {
                if Path::new(path).exists() {
                    info!("CUDA libraries detected at {}", path);
                    return true;
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            use std::path::Path;
            
            // Check common CUDA library locations on Linux
            let cuda_paths = [
                "/usr/local/cuda/lib64/libcudart.so",
                "/usr/local/cuda-11.0/lib64/libcudart.so",
                "/usr/local/cuda-11.1/lib64/libcudart.so",
                "/usr/local/cuda-11.2/lib64/libcudart.so",
                "/usr/local/cuda-11.3/lib64/libcudart.so",
                "/usr/local/cuda-11.4/lib64/libcudart.so",
                "/usr/local/cuda-11.5/lib64/libcudart.so",
                "/usr/local/cuda-11.6/lib64/libcudart.so",
                "/usr/local/cuda-11.7/lib64/libcudart.so",
                "/usr/local/cuda-11.8/lib64/libcudart.so",
                "/usr/local/cuda-12.0/lib64/libcudart.so",
                "/usr/local/cuda-12.1/lib64/libcudart.so",
                "/usr/local/cuda-12.2/lib64/libcudart.so",
            ];
            
            for path in cuda_paths {
                if Path::new(path).exists() {
                    info!("CUDA libraries detected at {}", path);
                    return true;
                }
            }
        }
        
        // If we get here, we couldn't find CUDA
        warn!("CUDA feature is enabled but no CUDA installation detected");
        return false;
    }
    
    #[cfg(not(feature = "cuda"))]
    {
        info!("CUDA feature is not enabled in this build");
        return false;
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
    
    // Check if we should force CPU mode
    let config = get_config();
    let force_cpu = config.force_cpu;
    
    // Check CUDA availability if not forcing CPU
    let cuda_available = if force_cpu {
        info!("Forcing CPU mode as requested in configuration");
        false
    } else {
        let available = check_cuda_availability();
        CUDA_AVAILABLE.store(available, Ordering::SeqCst);
        
        if available {
            info!("CUDA is available, attempting to use GPU acceleration");
        } else {
            info!("CUDA is not available, using CPU mode");
        }
        
        available
    };
    
    // First try with GPU if available and not forcing CPU
    if cuda_available && !force_cpu {
        info!("Attempting to load model with GPU acceleration...");
        
        let mut params = WhisperContextParameters::default();
        params.use_gpu(true); // Explicitly enable GPU if available
        
        match WhisperContext::new_with_params(
            &model_path.to_string_lossy(),
            params
        ) {
            Ok(ctx) => {
                info!("Successfully loaded model with GPU acceleration");
                *context = Some(ctx);
                USING_CPU_FALLBACK.store(false, Ordering::SeqCst);
            },
            Err(e) => {
                warn!("Failed to load model with GPU acceleration: {}", e);
                warn!("Falling back to CPU mode");
                
                // Try again with CPU
                match WhisperContext::new_with_params(
                    &model_path.to_string_lossy(),
                    WhisperContextParameters::default()
                ) {
                    Ok(ctx) => {
                        info!("Successfully loaded model in CPU fallback mode");
                        *context = Some(ctx);
                        USING_CPU_FALLBACK.store(true, Ordering::SeqCst);
                    },
                    Err(e) => {
                        error!("Failed to load model even in CPU fallback mode: {}", e);
                        return Err(AppError::Whisper(format!("Failed to load model: {}", e)).into());
                    }
                }
            }
        }
    } else {
        // Directly use CPU mode
        info!("Loading model in CPU mode...");
        
        match WhisperContext::new_with_params(
            &model_path.to_string_lossy(),
            WhisperContextParameters::default()
        ) {
            Ok(ctx) => {
                info!("Successfully loaded model in CPU mode");
                *context = Some(ctx);
                USING_CPU_FALLBACK.store(true, Ordering::SeqCst);
            },
            Err(e) => {
                error!("Failed to load model in CPU mode: {}", e);
                return Err(AppError::Whisper(format!("Failed to load model: {}", e)).into());
            }
        }
    }
    
    // Update the model size used
    let mut current_model = MODEL_SIZE_USED.lock();
    *current_model = model_size.to_string();
    
    info!("Whisper model {} loaded successfully", model_size);
    Ok(())
}

/// Returns true if the model is currently using CPU fallback mode
pub fn is_using_cpu_fallback() -> bool {
    USING_CPU_FALLBACK.load(Ordering::SeqCst)
}

/// Returns true if CUDA is available on the system
pub fn is_cuda_available() -> bool {
    CUDA_AVAILABLE.load(Ordering::SeqCst)
}

pub fn transcribe_audio(audio_data: &[f32]) -> Result<String> {
    info!("Starting whisper transcription with {} audio samples", audio_data.len());
    
    if audio_data.is_empty() {
        error!("Empty audio data passed to transcribe_audio");
        return Err(AppError::Whisper("Empty audio data".to_string()).into());
    }
    
    // Check if model is loaded
    let context = WHISPER_CONTEXT.lock();
    if context.is_none() {
        error!("Whisper model not loaded");
        return Err(AppError::Whisper("Whisper model not loaded".to_string()).into());
    }
    
    let ctx = context.as_ref().unwrap();
    info!("Whisper model is loaded and ready");
    
    // Create params
    info!("Creating Whisper parameters");
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("en"));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_timestamps(false);
    
    // Add a progress callback to yield CPU and GPU resources periodically
    params.set_progress_callback(|_progress| {
        // Yield to other threads periodically to maintain system responsiveness
        std::thread::yield_now();
        
        // More aggressive GPU yielding - longer sleep to allow GPU context switching
        std::thread::sleep(std::time::Duration::from_millis(1));
        
        // Force thread context switch on Windows for better system responsiveness
        #[cfg(target_os = "windows")]
        unsafe {
            use windows_sys::Win32::System::Threading::SwitchToThread;
            SwitchToThread();
        }
    });

    // Create state
    info!("Creating Whisper state");
    let mut state = match ctx.create_state() {
        Ok(state) => state,
        Err(e) => {
            error!("Failed to create Whisper state: {}", e);
            return Err(AppError::Whisper(format!("Failed to create state: {}", e)).into());
        }
    };

    // Run full transcription
    info!("Running full transcription process");
    if let Err(e) = state.full(params, audio_data) {
        error!("Failed to process audio with Whisper: {}", e);
        return Err(AppError::Whisper(format!("Failed to process audio: {}", e)).into());
    }

    // Get number of segments
    info!("Getting number of segments");
    let num_segments = match state.full_n_segments() {
        Ok(segments) => {
            info!("Transcription produced {} segments", segments);
            segments
        },
        Err(e) => {
            error!("Failed to get number of segments: {}", e);
            return Err(AppError::Whisper(format!("Failed to get number of segments: {}", e)).into());
        }
    };

    // Extract text from segments
    info!("Extracting text from {} segments", num_segments);
    let mut text = String::new();
    for i in 0..num_segments {
        match state.full_get_segment_text(i) {
            Ok(segment) => {
                info!("Segment {}: '{}'", i, segment);
                text.push_str(&segment);
            },
            Err(e) => {
                warn!("Failed to get text for segment {}: {}", i, e);
                // Continue with other segments rather than failing
            }
        }
    }

    if text.trim().is_empty() {
        warn!("Transcription produced empty text");
    } else {
        info!("Transcription complete: '{}'", text.trim());
    }

    Ok(text.trim().to_string())
} 