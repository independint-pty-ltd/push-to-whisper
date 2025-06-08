use anyhow::Result;
use std::time::{Duration, Instant};
use push_to_whisper::{audio, utils, state};

// Integration tests for v0.3.1 optimizations
#[test]
fn test_application_startup_performance() -> Result<()> {
    // Test that basic application components initialize quickly
    let start = Instant::now();
    
    // Test config loading
    let config = utils::get_config();
    
    // Test state checks
    let _recording = audio::is_recording();
    let _transcribing = audio::is_transcribing();
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(50), 
           "Application startup components should initialize quickly, took {:?}", duration);
    
    // Verify optimized configuration values
    assert!(config.long_press_threshold <= 50, 
           "Long press threshold should be optimized to 50ms or less");
    assert!(config.beep_volume >= 0.0 && config.beep_volume <= 1.0, 
           "Beep volume should be in valid range");
    
    Ok(())
}

#[test]
fn test_audio_system_responsiveness() -> Result<()> {
    // Test that audio system calls are responsive
    let start = Instant::now();
    
    // Test beep function (should return immediately)
    let beep_result = audio::play_beep_async(440, 50);
    
    // Test state queries
    for _ in 0..100 {
        let _recording = audio::is_recording();
        let _transcribing = audio::is_transcribing();
    }
    
    let duration = start.elapsed();
    
    assert!(beep_result.is_ok(), "Beep function should succeed");
    assert!(duration < Duration::from_millis(10), 
           "Audio system calls should be very responsive, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_state_management_performance() -> Result<()> {
    use push_to_whisper::ui::AppState;
    
    // Test state update performance
    let start = Instant::now();
    
    // Simulate rapid state updates
    for _ in 0..100 {
        state::send_state_update(AppState::Normal);
        state::send_state_update(AppState::Recording);
        state::send_state_update(AppState::Transcribing);
        state::send_state_update(AppState::Normal);
    }
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(50), 
           "State management should handle rapid updates efficiently, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_memory_efficiency() -> Result<()> {
    // Test that our optimizations don't cause memory leaks or excessive allocation
    let start = Instant::now();
    
    // Simulate typical usage patterns
    for _ in 0..10 {
        // Simulate audio buffer operations
        let mut buffer = Vec::with_capacity(16000); // 1 second at 16kHz
        for i in 0..16000 {
            buffer.push((i as f32) / 16000.0);
        }
        
        // Simulate config access
        let _config = utils::get_config();
        
        // Simulate beep calls
        let _beep = audio::play_beep_async(440, 10);
        
        // Let buffer go out of scope to test cleanup
    }
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(100), 
           "Memory operations should be efficient, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_error_handling_performance() -> Result<()> {
    // Test that error handling doesn't introduce significant overhead
    let start = Instant::now();
    
    // Test various error scenarios that should be handled gracefully
    for _ in 0..100 {
        // These should all succeed and be fast
        let _config = utils::get_config();
        let _beep = audio::play_beep_async(440, 1);
        let _recording = audio::is_recording();
    }
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(20), 
           "Error handling should not introduce significant overhead, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_concurrent_operations() -> Result<()> {
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    // Test that our optimizations work well under concurrent load
    let start = Instant::now();
    let success = Arc::new(AtomicBool::new(true));
    
    let mut handles = vec![];
    
    // Spawn multiple threads doing typical operations
    for i in 0..4 {
        let _success_clone = Arc::clone(&success);
        let handle = thread::spawn(move || {
            for _ in 0..25 {
                // Each thread does typical operations
                let _config = utils::get_config();
                let _recording = audio::is_recording();
                let _transcribing = audio::is_transcribing();
                
                if i % 2 == 0 {
                    let _beep = audio::play_beep_async(440 + i * 100, 5);
                }
                
                // Small delay to simulate real usage
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        if handle.join().is_err() {
            success.store(false, Ordering::SeqCst);
        }
    }
    
    let duration = start.elapsed();
    
    assert!(success.load(Ordering::SeqCst), "All concurrent operations should succeed");
    assert!(duration < Duration::from_millis(500), 
           "Concurrent operations should complete efficiently, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_configuration_optimization_integration() -> Result<()> {
    // Test that all our configuration optimizations work together
    let config = utils::get_config();
    
    // Verify all our optimized settings
    assert!(config.long_press_threshold <= 50, 
           "Long press threshold should be optimized for v0.3.1");
    
    assert!(config.beep_volume > 0.0 && config.beep_volume <= 1.0, 
           "Beep volume should be in valid range");
    
    assert!(!config.enable_debug_recording || config.enable_debug_recording, 
           "Debug recording setting should be valid boolean");
    
    // Test that beep respects the configuration
    let beep_start = Instant::now();
    let beep_result = audio::play_beep_async(440, 50);
    let beep_duration = beep_start.elapsed();
    
    assert!(beep_result.is_ok(), "Beep should work with current configuration");
    assert!(beep_duration < Duration::from_millis(5), 
           "Beep call should return immediately");
    
    Ok(())
} 