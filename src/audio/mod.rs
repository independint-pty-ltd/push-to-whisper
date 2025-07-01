use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound;
use log::{error, info, debug, warn};
use parking_lot::Mutex;
use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::sync::atomic::{AtomicBool, Ordering, AtomicU64};
use std::thread;
use std::time::Duration;
use once_cell::sync::Lazy;

use crate::error::AppError;
use crate::utils::get_config;
use crate::state::send_state_update;
use crate::state::RECORDING;
use crate::ui::AppState;

// Configuration
const SAMPLE_RATE: u32 = 16000;
const CHANNELS: u16 = 1;
const KEEP_HEADPHONES_ALIVE: bool = true;
const HEADPHONE_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(30);

#[derive(Debug, Clone)]
#[allow(dead_code)] // Configuration struct - fields may be used in future
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub keep_headphones_alive: bool,
    pub headphone_keepalive_interval: Duration,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            channels: CHANNELS,
            keep_headphones_alive: KEEP_HEADPHONES_ALIVE,
            headphone_keepalive_interval: HEADPHONE_KEEPALIVE_INTERVAL,
        }
    }
}

// Global state
static TRANSCRIBING: AtomicBool = AtomicBool::new(false);
static AUDIO_BUFFER: Lazy<Mutex<Vec<f32>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(SAMPLE_RATE as usize * 300))); // 5 minutes buffer

// Helper function to update activity time
fn update_activity_time() {
    crate::input::update_activity_time();
}

// Helper function to play a beep asynchronously
pub fn play_beep_async(frequency: u32, duration_ms: u64) -> Result<()> {
    // Check if beeps are disabled in config
    let config = get_config();
    if config.disable_beep {
        return Ok(());
    }

    let beep_volume = config.beep_volume;

    // Spawn a new thread to handle blocking audio playback
    thread::spawn(move || {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                let source = SineWave::new(frequency as f32)
                    .take_duration(Duration::from_millis(duration_ms))
                    .amplify(beep_volume);

                sink.append(source);
                sink.set_volume(beep_volume);
                sink.sleep_until_end();
            }
        }
    });

    Ok(())
}

#[allow(dead_code)] // May be used for debugging or future features
pub fn is_recording() -> bool {
    RECORDING.load(Ordering::SeqCst)
}

#[allow(dead_code)] // May be used for debugging or future features
pub fn is_transcribing() -> bool {
    TRANSCRIBING.load(Ordering::SeqCst)
}

pub fn start_recording() -> Result<()> {
    update_activity_time();
    
    if RECORDING.load(Ordering::SeqCst) {
        return Ok(());
    }
    
    AUDIO_BUFFER.lock().clear();
    
    info!("▶️ STARTING RECORDING ▶️");
    
    // Start recording immediately
    RECORDING.store(true, Ordering::SeqCst);
    std::thread::spawn(|| {
        if let Err(e) = audio_recording_thread() {
            error!("Audio recording thread error: {}", e);
            RECORDING.store(false, Ordering::SeqCst);
        }
    });

    // Get config to check if beeps are enabled
    let config = get_config();
    let beeps_enabled = !config.disable_beep;
    
    // Play beep asynchronously (non-blocking)
    if beeps_enabled {
        let _ = play_beep_async(1000, 100); // Reduced duration to 100ms for snappier feel
    }
    
    Ok(())
}

pub fn stop_recording() -> Result<()> {
    update_activity_time();
    
    if !RECORDING.load(Ordering::SeqCst) {
        info!("Stop recording called but we weren't recording");
        return Ok(());
    }
    
    info!("⏹️ STOPPING RECORDING ⏹️");
    RECORDING.store(false, Ordering::SeqCst);
    
    // Play stop beep asynchronously (non-blocking)
    let config = get_config();
    if !config.disable_beep {
        let _ = play_beep_async(800, 100); // Reduced duration to 100ms for snappier feel
    }
    
    // Get the recorded audio immediately (no artificial delay)
    let audio_data = AUDIO_BUFFER.lock().clone();
    
    if audio_data.is_empty() {
        warn!("No audio data was recorded (buffer is empty)");
        send_state_update(AppState::Normal);
        return Ok(());
    }
    
    info!("Captured {} audio samples", audio_data.len());
    
    // Get the device's native configuration
    let device = match cpal::default_host().default_input_device() {
        Some(device) => {
            info!("Found input device: {}", device.name().unwrap_or_default());
            device
        },
        None => {
            warn!("No input device available, assuming default configuration");
            // Continue with default values
            info!("Attempting transcription with default configuration (48000Hz, 1 channel)");
            let audio_data = process_audio_for_whisper(audio_data, 48000, 1)?;
            return process_transcription(audio_data);
        }
    };
    
    let default_config = match device.default_input_config() {
        Ok(config) => {
            info!("Got device config: {} channels at {} Hz", 
                 config.channels(), config.sample_rate().0);
            config
        },
        Err(e) => {
            warn!("Failed to get default input config: {}, assuming default configuration", e);
            // Continue with default values
            info!("Attempting transcription with default configuration (48000Hz, 1 channel)");
            let audio_data = process_audio_for_whisper(audio_data, 48000, 1)?;
            return process_transcription(audio_data);
        }
    };
    
    let device_sample_rate = default_config.sample_rate().0;
    let device_channels = default_config.channels();
    
    info!("Device configuration: {} channels at {} Hz", device_channels, device_sample_rate);
    
    // Process the audio for Whisper
    info!("Pre-processing audio data for transcription");
    let audio_data = match process_audio_for_whisper(audio_data, device_sample_rate, device_channels) {
        Ok(processed) => {
            info!("Audio successfully processed, ready for transcription ({} samples)", processed.len());
            processed
        },
        Err(e) => {
            error!("Failed to process audio for transcription: {}", e);
            send_state_update(AppState::Normal); // Revert to Normal on error
            return Err(e);
        }
    };
    
    // Send Transcribing state update BEFORE starting transcription
    info!("Setting application state to Transcribing");
    send_state_update(AppState::Transcribing);
    
    // Set the atomic flag after sending the update
    TRANSCRIBING.store(true, Ordering::SeqCst);
    
    // Process with Whisper in a separate thread to avoid blocking
    info!("Starting transcription with Whisper in background thread");
    
    // Clone the audio data for the background thread
    let audio_data_clone = audio_data.clone();
    let transcription_handle = thread::spawn(move || {
        // Set lower thread priority to prevent system lag
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::System::Threading::{GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_LOWEST, THREAD_PRIORITY_BELOW_NORMAL, THREAD_PRIORITY_NORMAL};
            
            // Get transcription priority from config
            let config = crate::utils::get_config();
            let priority = match config.transcription_priority.as_str() {
                "low" => THREAD_PRIORITY_LOWEST,
                "normal" => THREAD_PRIORITY_BELOW_NORMAL,
                "high" => THREAD_PRIORITY_NORMAL,
                _ => THREAD_PRIORITY_LOWEST, // Default to low
            };
            
            unsafe {
                SetThreadPriority(GetCurrentThread(), priority);
            }
            
            info!("Set transcription thread priority to: {}", config.transcription_priority);
        }
        
        #[cfg(target_os = "linux")]
        {
            // On Linux, we could use nice() or setpriority(), but it requires additional dependencies
            // For now, we'll rely on the thread scheduler
        }
        
        process_transcription(audio_data_clone)
    });
    
    // Wait for transcription to complete
    let transcription_result = match transcription_handle.join() {
        Ok(result) => result,
        Err(_) => {
            error!("Transcription thread panicked");
            Err(anyhow::anyhow!("Transcription thread panicked"))
        }
    };
    
    info!("Transcription process completed with result: {:?}", transcription_result.is_ok());
    
    // Set Transcribing flag to false AFTER processing completes
    TRANSCRIBING.store(false, Ordering::SeqCst);
    
    // Send Normal state update AFTER transcription finishes (or fails)
    send_state_update(AppState::Normal);
    
    transcription_result // Return the result of process_transcription
}

// Helper function to process audio for Whisper
fn process_audio_for_whisper(audio_data: Vec<f32>, sample_rate: u32, channels: u16) -> Result<Vec<f32>> {
    // Convert multi-channel audio to mono if needed
    let audio_data = if channels > 1 {
        info!("Converting {}-channel audio to mono", channels);
        let samples_per_frame = channels as usize;
        let frame_count = audio_data.len() / samples_per_frame;
        let mut mono_data = Vec::with_capacity(frame_count);
        
        for frame_idx in 0..frame_count {
            let start_idx = frame_idx * samples_per_frame;
            let end_idx = start_idx + samples_per_frame;
            
            if end_idx <= audio_data.len() {
                // Average all channels to create mono
                let frame_sum: f32 = audio_data[start_idx..end_idx].iter().sum();
                let mono_sample = frame_sum / samples_per_frame as f32;
                mono_data.push(mono_sample);
            }
        }
        
        info!("Converted from {} multi-channel samples to {} mono samples", 
              audio_data.len(), mono_data.len());
        mono_data
    } else {
        audio_data
    };
    
    // If the device sample rate is different from what Whisper expects, resample
    let audio_data = if sample_rate != SAMPLE_RATE {
        info!("Resampling audio from {}Hz to {}Hz", sample_rate, SAMPLE_RATE);
        
        // Simple linear interpolation resampling
        let resampling_ratio = sample_rate as f32 / SAMPLE_RATE as f32;
        let target_len = (audio_data.len() as f32 / resampling_ratio) as usize;
        let mut resampled = Vec::with_capacity(target_len);
        
        // Simple resampling by linear interpolation
        for i in 0..target_len {
            let src_idx = i as f32 * resampling_ratio;
            let src_idx_floor = src_idx.floor() as usize;
            let src_idx_ceil = src_idx.ceil() as usize;
            let t = src_idx - src_idx_floor as f32;
            
            if src_idx_ceil >= audio_data.len() {
                break;
            }
            
            let sample = audio_data[src_idx_floor] * (1.0 - t) + audio_data[src_idx_ceil] * t;
            resampled.push(sample);
        }
        
        info!("Resampled from {} to {} samples", audio_data.len(), resampled.len());
        resampled
    } else {
        audio_data
    };
    
    // For debugging, save the audio to a file
    let debug_wav_path = "debug_recording.wav";
    
    // Only save debug recording if enabled in config
    if crate::utils::get_config().enable_debug_recording {
        match save_debug_audio(&audio_data, debug_wav_path) {
            Ok(_) => info!("Saved debug recording to {}", debug_wav_path),
            Err(e) => warn!("Failed to save debug recording: {}", e),
        }
    }
    
    Ok(audio_data)
}

// Helper function to process transcription
fn process_transcription(audio_data: Vec<f32>) -> Result<()> {
    // Ensure TRANSCRIBING flag is true at the start
    if !TRANSCRIBING.load(Ordering::SeqCst) {
        warn!("process_transcription called but TRANSCRIBING flag is false");
        // Optionally set it true here, or return error?
        TRANSCRIBING.store(true, Ordering::SeqCst);
        info!("Set TRANSCRIBING flag to true");
    }
    
    info!("Processing speech with Whisper... (audio length: {} samples)", audio_data.len());
    
    // Save a copy of the audio data for debugging
    if crate::utils::get_config().enable_debug_recording {
        match save_debug_audio(&audio_data, "transcription_input.wav") {
            Ok(_) => info!("Saved transcription input to transcription_input.wav"),
            Err(e) => warn!("Failed to save transcription input: {}", e),
        }
    }

    // Make sure we're actually passing data to whisper
    if audio_data.is_empty() {
        error!("No audio data to transcribe (empty buffer)");
        return Err(anyhow::anyhow!("No audio data to transcribe"));
    }

    info!("Calling whisper::transcribe_audio with {} samples", audio_data.len());
    let result = match crate::whisper::transcribe_audio(&audio_data) {
        Ok(text) => Ok(text),
        Err(e) => {
            error!("Whisper transcription failed: {}", e);
            Err(e)
        }
    };

    match result {
        Ok(text) => {
            info!("Transcription successful - result: '{}'", text);
            if !text.trim().is_empty() {
                info!("Attempting to insert transcribed text");
                crate::input::type_text(&text)?;
            } else {
                warn!("Transcription returned empty text - nothing to insert");
            }
            Ok(())
        }
        Err(e) => {
            error!("Transcription failed with error: {}", e);
            Err(e)
        }
    }
}

fn audio_recording_thread() -> Result<()> {
    debug!("Initializing audio recording thread");
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| AppError::Device("No input device available".to_string()))?;

    debug!("Using input device: {}", device.name()?);

    // Get the device's default config and try it first
    let default_config = device.default_input_config()
        .map_err(|e| AppError::Device(format!("Failed to get default input config: {}", e)))?;

    // Try the native configuration first
    if try_build_stream(&device, &default_config).is_ok() {
        return Ok(());
    }
    
    warn!("Failed to build stream with native config, trying fallbacks");
    
    // Quick fallback to common configurations
    let fallback_configs = [
        (1, 44100),  // Mono, 44.1kHz (most common)
        (1, 48000),  // Mono, 48kHz 
        (2, 44100),  // Stereo, 44.1kHz
        (1, 16000),  // Mono, 16kHz (speech optimized)
    ];
    
    for (channels, sample_rate) in fallback_configs.iter() {
        let config = cpal::StreamConfig {
            channels: *channels,
            sample_rate: cpal::SampleRate(*sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };
        
        if try_build_input_stream(&device, &config).is_ok() {
            return Ok(());
        }
    }
    
    Err(anyhow::anyhow!("Failed to initialize audio stream with any configuration"))
}

// Helper function to try building a stream with a specific configuration
fn try_build_stream(device: &cpal::Device, config: &cpal::SupportedStreamConfig) -> Result<()> {
    let stream_config: cpal::StreamConfig = config.clone().into();
    try_build_input_stream(device, &stream_config)
}

// Helper function to try building a stream with a specific StreamConfig
fn try_build_input_stream(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<()> {
    debug!("Trying stream config: {} channels at {} Hz", 
           config.channels, 
           config.sample_rate.0);

    let err_fn = move |err| {
        error!("An error occurred on stream: {}", err);
    };

    // Reset the audio buffer before starting
    AUDIO_BUFFER.lock().clear();
    debug!("Audio buffer cleared, ready for new data");

    debug!("Building input stream...");
    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if RECORDING.load(Ordering::SeqCst) {
                // Debug log only for the first callback to avoid flooding
                static CALLBACK_COUNT: AtomicU64 = AtomicU64::new(0);
                let count = CALLBACK_COUNT.fetch_add(1, Ordering::SeqCst);
                if count == 0 {
                    debug!("Audio callback started, receiving {} samples per callback", data.len());
                }
                
                // Add the data to our buffer
                AUDIO_BUFFER.lock().extend_from_slice(data);
                
                // Log total sample count occasionally (less frequently)
                if count % 100 == 0 && count > 0 {
                    let buffer_size = AUDIO_BUFFER.lock().len();
                    debug!("Audio buffer now contains {} samples", buffer_size);
                }
            }
        },
        err_fn,
        None
    )?;

    debug!("Starting stream playback...");
    stream.play()?;

    debug!("Recording thread running...");
    // We need to keep the stream alive while recording
    while RECORDING.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(1)); // Reduced from 10ms to 1ms for faster response
    }

    // Log the final buffer size for debugging
    let final_size = AUDIO_BUFFER.lock().len();
    info!("Recording stopped. Final buffer contains {} samples", final_size);
    debug!("Recording thread finished");
    Ok(())
}

#[allow(dead_code)] // Alternative beep implementation
pub fn play_beep(frequency: u32, _duration_ms: u64) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    let source = SineWave::new(frequency as f32);
    sink.append(source);
    sink.sleep_until_end();
    
    Ok(())
}

pub fn headphone_keepalive_thread() -> Result<()> {
    if !KEEP_HEADPHONES_ALIVE {
        return Ok(());
    }

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    
    thread::spawn(move || {
        loop {
            let source = SineWave::new(20.0); // Very low frequency
            sink.append(source);
            sink.sleep_until_end();
            thread::sleep(HEADPHONE_KEEPALIVE_INTERVAL);
        }
    });

    Ok(())
}

// Function to list available audio devices for troubleshooting
pub fn list_audio_devices() -> Result<()> {
    let host = cpal::default_host();
    
    info!("Audio Host: {}", host.id().name());
    
    // List input devices
    match host.input_devices() {
        Ok(devices) => {
            let devices: Vec<_> = devices.collect();
            if devices.is_empty() {
                info!("No input devices found");
            } else {
                info!("Available input devices:");
                for (i, device) in devices.iter().enumerate() {
                    match device.name() {
                        Ok(name) => info!("  {}. {}", i + 1, name),
                        Err(_) => info!("  {}. <unknown name>", i + 1),
                    }
                    
                    // Try to get supported configs
                    match device.supported_input_configs() {
                        Ok(configs) => {
                            let configs: Vec<_> = configs.collect();
                            if configs.is_empty() {
                                info!("     No supported configurations found");
                            } else {
                                for (j, config) in configs.iter().enumerate() {
                                    info!("     {}.{} Channels: {}, Sample rates: {} - {} Hz, Format: {:?}",
                                          i + 1, j + 1,
                                          config.channels(),
                                          config.min_sample_rate().0,
                                          config.max_sample_rate().0,
                                          config.sample_format());
                                }
                            }
                        },
                        Err(e) => info!("     Error getting supported configs: {}", e),
                    }
                }
            }
        },
        Err(e) => warn!("Error enumerating input devices: {}", e),
    }
    
    // Get default input device
    match host.default_input_device() {
        Some(device) => {
            match device.name() {
                Ok(name) => info!("Default input device: {}", name),
                Err(_) => info!("Default input device: <unknown name>"),
            }
        },
        None => info!("No default input device found"),
    }
    
    Ok(())
}

pub fn save_debug_audio(audio_data: &[f32], path: &str) -> Result<()> {
    // Always save as mono audio (Whisper expects mono)
    let spec = hound::WavSpec {
        channels: 1, // Always mono
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;
    
    for &sample in audio_data {
        writer.write_sample(sample)?;
    }
    
    writer.finalize()?;
    Ok(())
} 