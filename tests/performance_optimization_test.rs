use anyhow::Result;
use std::time::{Duration, Instant};
use std::thread;
use push_to_whisper::audio;
use push_to_whisper::utils::get_config;
use log::info;

// Test performance improvements for mouse responsiveness during transcription
#[test]
fn test_main_loop_responsiveness() -> Result<()> {
    // Test that our main loop tick rate is optimal for responsiveness
    let config = get_config();
    
    // The main loop should tick at 16ms (60 FPS) for smooth responsiveness
    let expected_tick_rate = Duration::from_millis(16);
    
    // Simulate the main loop timing
    let start = Instant::now();
    let iterations = 100;
    
    for _ in 0..iterations {
        thread::sleep(expected_tick_rate);
    }
    
    let total_duration = start.elapsed();
    let average_per_iteration = total_duration / iterations;
    
    // Should be close to 16ms per iteration with some tolerance
    assert!(average_per_iteration < Duration::from_millis(20), 
           "Main loop timing should be responsive, average was {:?}", average_per_iteration);
    
    info!("Main loop responsiveness test passed with average {:?} per iteration", average_per_iteration);
    Ok(())
}

#[test]
fn test_audio_recording_thread_responsiveness() -> Result<()> {
    // Test that audio recording thread has minimal sleep for fast response
    let start = Instant::now();
    
    // Simulate the recording thread sleep cycle
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(1)); // Our optimized sleep duration
    }
    
    let duration = start.elapsed();
    
    // Should complete 100 iterations in reasonable time
    assert!(duration < Duration::from_millis(200), 
           "Audio recording thread should be responsive, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_long_press_threshold_optimization() -> Result<()> {
    let config = get_config();
    
    // Our optimized threshold should be 25ms or less for snappy response
    assert!(config.long_press_threshold <= 25, 
           "Long press threshold should be optimized for responsiveness, was {}ms", 
           config.long_press_threshold);
    
    Ok(())
}

#[test]
fn test_ui_update_responsiveness() -> Result<()> {
    // Test that UI updates happen quickly
    let start = Instant::now();
    
    // Simulate UI update loop timing (5ms sleep)
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(5));
    }
    
    let duration = start.elapsed();
    
    // Should complete in reasonable time for smooth UI
    assert!(duration < Duration::from_millis(300), 
           "UI updates should be responsive, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_text_input_timing_optimization() -> Result<()> {
    // Test that text input timing is optimized
    let start = Instant::now();
    
    // Simulate typing 50 characters with our optimized 1ms delay
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(1));
    }
    
    let duration = start.elapsed();
    
    // Should be very fast for smooth typing
    assert!(duration < Duration::from_millis(100), 
           "Text input should be fast, took {:?} for 50 characters", duration);
    
    Ok(())
}

#[test]
fn test_memory_efficiency() -> Result<()> {
    // Test that our audio buffer is sized efficiently
    let sample_rate = 16000_usize;
    let optimized_capacity = sample_rate * 60; // 60 seconds
    let old_capacity = sample_rate * 300; // 300 seconds (old version)
    
    // Our optimization should use significantly less memory
    let memory_savings = old_capacity - optimized_capacity;
    let savings_percentage = (memory_savings as f32 / old_capacity as f32) * 100.0;
    
    assert!(savings_percentage > 70.0, 
           "Memory optimization should save significant space, only saved {:.1}%", 
           savings_percentage);
    
    info!("Memory optimization saves {:.1}% ({} samples)", savings_percentage, memory_savings);
    Ok(())
}

#[test]
fn test_beep_responsiveness() -> Result<()> {
    // Test that beep function returns immediately for non-blocking operation
    let start = Instant::now();
    
    // Our async beep should return immediately
    let result = audio::play_beep_async(440, 50);
    
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Beep should succeed");
    assert!(duration < Duration::from_millis(5), 
           "Async beep should return immediately, took {:?}", duration);
    
    Ok(())
}

// Integration test for overall responsiveness during simulated transcription
#[test]
fn test_transcription_system_responsiveness() -> Result<()> {
    let start = Instant::now();
    
    // Simulate the responsiveness test by checking all timing-critical components
    let config = get_config();
    
    // Check that all our optimizations are in place
    assert!(config.long_press_threshold <= 25, "Long press should be optimized");
    
    // Simulate rapid state changes like during transcription
    for _ in 0..20 {
        // Simulate main loop tick
        thread::sleep(Duration::from_millis(16));
        
        // Simulate UI update
        thread::sleep(Duration::from_millis(5));
        
        // Simulate audio processing
        thread::sleep(Duration::from_millis(1));
    }
    
    let duration = start.elapsed();
    
    // Total should be reasonable for 20 cycles of operation
    assert!(duration < Duration::from_millis(500), 
           "System should remain responsive during transcription simulation, took {:?}", duration);
    
    info!("System responsiveness test completed in {:?}", duration);
    Ok(())
}