use anyhow::{Result, Context};
use std::path::Path;
use std::fs;
use push_to_whisper::audio;
use log::info;

// This test module focuses on audio functionality without requiring actual microphone input
// It uses mocked audio data to test the processing pipeline

// Test constants
const TEST_AUDIO_DIR: &str = "tests/fixtures";
const TEST_OUTPUT_FILE: &str = "tests/fixtures/test_output.wav";

// Setup function to ensure test directories exist
fn setup() -> Result<()> {
    if !Path::new(TEST_AUDIO_DIR).exists() {
        fs::create_dir_all(TEST_AUDIO_DIR)?;
    }
    Ok(())
}

// This test checks the audio processing functionality
#[test]
fn test_audio_processing() -> Result<()> {
    setup()?;
    
    // Generate a test sine wave
    let sample_rate = 16000;
    let duration_secs = 1.0;
    let frequency = 440.0; // A4 note
    
    let audio_data = generate_sine_wave(sample_rate, duration_secs, frequency);
    assert_eq!(audio_data.len(), (sample_rate as f64 * duration_secs) as usize);
    
    // Save the audio to a file
    save_audio_to_wav(&audio_data, sample_rate, TEST_OUTPUT_FILE)?;
    
    // Verify the file exists
    assert!(Path::new(TEST_OUTPUT_FILE).exists());
    
    // Calculate RMS of the audio (should be around 0.707 for a sine wave)
    let rms = calculate_rms(&audio_data);
    assert!(rms > 0.6 && rms < 0.8, "RMS should be approximately 0.707 for a sine wave");
    
    // Test the beep function (just make sure it doesn't crash)
    // This is a non-blocking test that doesn't actually play sound
    let result = audio::play_beep_async(440, 10); // Use the new async function
    assert!(result.is_ok(), "play_beep_async should not fail immediately");
    
    // Clean up
    if Path::new(TEST_OUTPUT_FILE).exists() {
        fs::remove_file(TEST_OUTPUT_FILE)?;
    }
    
    Ok(())
}

// Generate a sine wave for testing
fn generate_sine_wave(sample_rate: u32, duration_secs: f64, frequency: f64) -> Vec<f32> {
    let num_samples = (sample_rate as f64 * duration_secs) as usize;
    let mut audio_data = Vec::with_capacity(num_samples);
    
    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let sample = (2.0 * std::f64::consts::PI * frequency * t).sin() as f32;
        audio_data.push(sample);
    }
    
    audio_data
}

// Calculate RMS (Root Mean Square) of audio samples
fn calculate_rms(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    
    let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
    (sum_squares / samples.len() as f32).sqrt()
}

// Save audio data to a WAV file
fn save_audio_to_wav(audio_data: &[f32], sample_rate: u32, path: &str) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    
    let mut writer = hound::WavWriter::create(path, spec)
        .context("Failed to create WAV file")?;
    
    for sample in audio_data {
        writer.write_sample(*sample)
            .context("Failed to write sample to WAV file")?;
    }
    
    writer.finalize().context("Failed to finalize WAV file")?;
    
    Ok(())
}

#[test]
fn test_play_beep_async() {
    // This test might be flaky in headless environments or CI without audio output
    // It primarily ensures the function call doesn't panic
    info!("Testing asynchronous beep playback...");
    let result = audio::play_beep_async(440, 10); // Use the new async function
    assert!(result.is_ok(), "play_beep_async should not return an error immediately");
    // We can't easily assert the beep played without more complex audio capture,
    // but we can wait a short time to allow the thread to potentially run and finish.
} 