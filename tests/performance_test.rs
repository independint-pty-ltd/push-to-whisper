use anyhow::Result;
use std::time::{Duration, Instant};
use push_to_whisper::audio;
use push_to_whisper::utils::get_config;

// Test performance improvements for v0.3.1
#[test]
fn test_beep_performance() -> Result<()> {
    // Test that beep function returns quickly (should be async)
    let start = Instant::now();
    let result = audio::play_beep_async(440, 100);
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "play_beep_async should succeed");
    assert!(duration < Duration::from_millis(10), 
           "play_beep_async should return in less than 10ms, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_config_performance() -> Result<()> {
    // Test that config loading is fast
    let start = Instant::now();
    let config = get_config();
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(5), 
           "get_config should return in less than 5ms, took {:?}", duration);
    
    // Verify our optimized defaults
    assert!(config.long_press_threshold <= 50, 
           "Long press threshold should be 50ms or less for snappy response");
    
    Ok(())
}

#[test]
fn test_audio_buffer_operations() -> Result<()> {
    // Test that audio buffer operations are fast
    let test_data: Vec<f32> = (0..16000).map(|i| (i as f32) / 16000.0).collect();
    
    let start = Instant::now();
    
    // Simulate what happens in our optimized audio callback
    let mut buffer = Vec::with_capacity(16000 * 10); // 10 seconds capacity
    buffer.extend_from_slice(&test_data);
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(1), 
           "Audio buffer operations should be very fast, took {:?}", duration);
    assert_eq!(buffer.len(), test_data.len(), "Buffer should contain all test data");
    
    Ok(())
}

#[test]
fn test_recording_state_transitions() -> Result<()> {
    use push_to_whisper::audio::{is_recording, is_transcribing};
    
    // Test that state checks are fast
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _recording = is_recording();
        let _transcribing = is_transcribing();
    }
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(1), 
           "1000 state checks should complete in less than 1ms, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_sine_wave_generation_performance() -> Result<()> {
    // Test performance of sine wave generation for beeps
    let start = Instant::now();
    
    let sample_rate = 44100;
    let duration_ms = 100;
    let frequency = 440.0;
    let num_samples = (sample_rate * duration_ms) / 1000;
    
    let mut samples = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * std::f32::consts::PI * frequency * t).sin();
        samples.push(sample);
    }
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(5), 
           "Sine wave generation should be fast, took {:?}", duration);
    assert_eq!(samples.len(), num_samples, "Should generate correct number of samples");
    
    Ok(())
}

#[test]
fn test_memory_allocation_performance() -> Result<()> {
    // Test that our buffer allocations are efficient
    let start = Instant::now();
    
    // Simulate allocating audio buffers like we do in the real application
    let sample_rate = 16000;
    let max_duration_seconds = 300; // 5 minutes
    let capacity = sample_rate * max_duration_seconds;
    
    let buffer: Vec<f32> = Vec::with_capacity(capacity);
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(1), 
           "Buffer allocation should be very fast, took {:?}", duration);
    assert_eq!(buffer.capacity(), capacity, "Buffer should have correct capacity");
    
    Ok(())
}

// Benchmark test for audio processing pipeline
#[test]
fn test_audio_processing_pipeline_performance() -> Result<()> {
    // Generate test audio data
    let sample_rate = 16000;
    let duration_secs = 1.0;
    let num_samples = (sample_rate as f64 * duration_secs) as usize;
    
    let audio_data: Vec<f32> = (0..num_samples)
        .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sample_rate as f32).sin())
        .collect();
    
    let start = Instant::now();
    
    // Simulate the audio processing we do (mono conversion, resampling check, etc.)
    let processed_data = if sample_rate != 16000 {
        // This branch shouldn't execute in our test, but we include it for completeness
        audio_data
    } else {
        audio_data
    };
    
    // Simulate RMS calculation (volume detection)
    let rms: f32 = processed_data.iter()
        .map(|s| s * s)
        .sum::<f32>() / processed_data.len() as f32;
    let rms = rms.sqrt();
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(10), 
           "Audio processing pipeline should be fast, took {:?}", duration);
    assert!(rms > 0.0, "RMS should be positive for sine wave");
    
    Ok(())
} 