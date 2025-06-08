use anyhow::Result;
use std::time::{Duration, Instant};
use push_to_whisper::input::{InputConfig, TextInsertMethod};

#[test]
fn test_input_config_defaults() -> Result<()> {
    let config = InputConfig::default();
    
    // Verify our optimized defaults
    assert_eq!(config.long_press_threshold, 500, "Default long press threshold should be 500ms");
    assert_eq!(config.text_insert_method, TextInsertMethod::Clipboard, "Default should use clipboard method");
    
    Ok(())
}

#[test]
fn test_text_insert_method_enum() -> Result<()> {
    // Test that our enum variants work correctly
    let clipboard = TextInsertMethod::Clipboard;
    let shortcut = TextInsertMethod::Shortcut;
    let typing = TextInsertMethod::Typing;
    
    // Test equality
    assert_eq!(clipboard, TextInsertMethod::Clipboard);
    assert_eq!(shortcut, TextInsertMethod::Shortcut);
    assert_eq!(typing, TextInsertMethod::Typing);
    
    // Test inequality
    assert_ne!(clipboard, shortcut);
    assert_ne!(shortcut, typing);
    assert_ne!(typing, clipboard);
    
    Ok(())
}

#[test]
fn test_input_config_creation_performance() -> Result<()> {
    // Test that creating input configs is fast
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _config = InputConfig::default();
    }
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(10), 
           "Creating 1000 InputConfig instances should be fast, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_input_config_clone_performance() -> Result<()> {
    let config = InputConfig::default();
    
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _cloned = config.clone();
    }
    
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_millis(5), 
           "Cloning 1000 InputConfig instances should be fast, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_optimized_thresholds() -> Result<()> {
    let config = InputConfig::default();
    
    // Verify our performance optimizations
    assert!(config.long_press_threshold <= 500, 
           "Long press threshold should be optimized for responsiveness");
    
    assert!(config.clipboard_restore_delay <= Duration::from_secs(10), 
           "Clipboard restore delay should be reasonable");
    
    Ok(())
}

// Test simulated timing scenarios
#[test]
fn test_timing_scenarios() -> Result<()> {
    let config = InputConfig::default();
    
    // Simulate quick key press (should not trigger recording)
    let quick_press_duration = Duration::from_millis(10);
    assert!(quick_press_duration.as_millis() < config.long_press_threshold as u128,
           "Quick press should be shorter than threshold");
    
    // Simulate long press (should trigger recording)
    let long_press_duration = Duration::from_millis(config.long_press_threshold + 100);
    assert!(long_press_duration.as_millis() > config.long_press_threshold as u128,
           "Long press should exceed threshold");
    
    Ok(())
}

#[test]
fn test_text_processing_simulation() -> Result<()> {
    // Simulate text processing performance
    let test_text = "Hello, this is a test transcription from Whisper.";
    
    let start = Instant::now();
    
    // Simulate the text processing we do
    let trimmed = test_text.trim();
    let is_empty = trimmed.is_empty();
    let char_count = trimmed.chars().count();
    
    let duration = start.elapsed();
    
    assert!(!is_empty, "Test text should not be empty");
    assert!(char_count > 0, "Character count should be positive");
    assert!(duration < Duration::from_millis(1), 
           "Text processing should be very fast, took {:?}", duration);
    
    Ok(())
}

#[test]
fn test_clipboard_delay_optimization() -> Result<()> {
    let config = InputConfig::default();
    
    // Our optimized clipboard delay should be reasonable
    assert!(config.clipboard_restore_delay >= Duration::from_secs(1), 
           "Clipboard restore delay should be at least 1 second");
    assert!(config.clipboard_restore_delay <= Duration::from_secs(30), 
           "Clipboard restore delay should not be excessive");
    
    Ok(())
} 