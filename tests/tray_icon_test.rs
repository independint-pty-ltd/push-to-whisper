use anyhow::Result;
use std::time::{Duration, Instant};
use push_to_whisper::ui::{AppState, init_tray_icon, update_tray_icon, process_menu_actions, cleanup_tray};

#[test]
fn test_tray_icon_initialization() -> Result<()> {
    // Test that tray icon initializes without errors
    let start = Instant::now();
    let result = init_tray_icon();
    let duration = start.elapsed();
    
    // Should initialize quickly
    assert!(duration < Duration::from_millis(1000), 
           "Tray icon initialization should be fast, took {:?}", duration);
    
    // Should succeed (or be disabled)
    assert!(result.is_ok(), "Tray icon initialization should succeed: {:?}", result);
    
    // Clean up
    cleanup_tray();
    
    Ok(())
}

#[test]
fn test_tray_icon_state_updates() -> Result<()> {
    // Initialize tray icon
    init_tray_icon()?;
    
    // Test state updates don't crash
    let states = [AppState::Normal, AppState::Recording, AppState::Transcribing];
    
    for state in &states {
        let start = Instant::now();
        update_tray_icon(*state);
        let duration = start.elapsed();
        
        // Updates should be fast
        assert!(duration < Duration::from_millis(100), 
               "Tray icon update should be fast, took {:?} for state {:?}", duration, state);
    }
    
    // Clean up
    cleanup_tray();
    
    Ok(())
}

#[test]
fn test_menu_actions_processing() -> Result<()> {
    // Initialize tray icon
    init_tray_icon()?;
    
    // Test that menu action processing doesn't crash
    let start = Instant::now();
    let should_exit = process_menu_actions()?;
    let duration = start.elapsed();
    
    // Should process quickly
    assert!(duration < Duration::from_millis(50), 
           "Menu action processing should be fast, took {:?}", duration);
    
    // Should not exit by default (no actions pending)
    assert!(!should_exit, "Should not exit when no menu actions are pending");
    
    // Clean up
    cleanup_tray();
    
    Ok(())
}

#[test]
fn test_app_state_enum() -> Result<()> {
    // Test that AppState enum works correctly
    let normal = AppState::Normal;
    let recording = AppState::Recording;
    let transcribing = AppState::Transcribing;
    
    // Test equality
    assert_eq!(normal, AppState::Normal);
    assert_eq!(recording, AppState::Recording);
    assert_eq!(transcribing, AppState::Transcribing);
    
    // Test inequality
    assert_ne!(normal, recording);
    assert_ne!(recording, transcribing);
    assert_ne!(transcribing, normal);
    
    // Test debug formatting
    let debug_str = format!("{:?}", normal);
    assert!(debug_str.contains("Normal"));
    
    Ok(())
}

#[test]
fn test_tray_icon_performance() -> Result<()> {
    // Test multiple rapid state changes
    init_tray_icon()?;
    
    let start = Instant::now();
    
    // Simulate rapid state changes
    for i in 0..10 {
        let state = match i % 3 {
            0 => AppState::Normal,
            1 => AppState::Recording,
            _ => AppState::Transcribing,
        };
        update_tray_icon(state);
    }
    
    let duration = start.elapsed();
    
    // Should handle rapid updates efficiently
    assert!(duration < Duration::from_millis(500), 
           "Rapid tray icon updates should be efficient, took {:?}", duration);
    
    // Clean up
    cleanup_tray();
    
    Ok(())
}

#[test]
fn test_tray_icon_cleanup() -> Result<()> {
    // Test that cleanup doesn't crash
    init_tray_icon()?;
    
    let start = Instant::now();
    cleanup_tray();
    let duration = start.elapsed();
    
    // Cleanup should be fast
    assert!(duration < Duration::from_millis(100), 
           "Tray icon cleanup should be fast, took {:?}", duration);
    
    // Multiple cleanups should not crash
    cleanup_tray();
    cleanup_tray();
    
    Ok(())
} 