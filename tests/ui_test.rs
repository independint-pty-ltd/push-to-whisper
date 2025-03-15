use anyhow::Result;
use push_to_whisper::ui;

// This test checks the UI module's tray icon functionality
// It's a basic test that just ensures the function doesn't crash
#[test]
fn test_tray_icon_update() -> Result<()> {
    // Test updating the tray icon
    // This is a simple test that just makes sure the function doesn't crash
    ui::update_tray_icon(false);
    ui::update_tray_icon(true);
    
    // If we got here without crashing, the test passes
    Ok(())
}

// This test is ignored by default as it requires a GUI environment
// Run with: cargo test --test ui_test -- --ignored test_tray_icon_init
#[test]
#[ignore]
fn test_tray_icon_init() -> Result<()> {
    // Initialize the tray icon
    // This is a more involved test that actually creates a tray icon
    // It's ignored by default because it requires a GUI environment
    let result = ui::init_tray_icon();
    assert!(result.is_ok(), "init_tray_icon should not fail");
    
    // Update the tray icon a few times
    ui::update_tray_icon(false);
    ui::update_tray_icon(true);
    ui::update_tray_icon(false);
    
    // If we got here without crashing, the test passes
    Ok(())
} 