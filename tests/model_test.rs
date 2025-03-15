use anyhow::Result;
use push_to_whisper::model;

// This test is ignored by default as it requires network access to download models
// Run with: cargo test --test model_test -- --ignored
#[tokio::test]
#[ignore]
async fn test_ensure_model_exists() -> Result<()> {
    // Use the tiny model for faster testing
    let model_size = "tiny.en";
    
    // Get the expected model path
    let expected_path = model::get_model_path(model_size);
    
    // If the model already exists, remove it to test the download functionality
    if expected_path.exists() {
        println!("Model already exists at {:?}, removing for test", expected_path);
        std::fs::remove_file(&expected_path)?;
    }
    
    // Ensure the model exists (should trigger a download)
    let model_path = model::ensure_model_exists(model_size).await?;
    
    // Verify the model was downloaded to the expected location
    assert!(model_path.exists(), "Model file should exist after download");
    assert_eq!(model_path, expected_path, "Model path should match expected path");
    
    // Verify the file size is reasonable (tiny.en is ~75MB)
    let metadata = std::fs::metadata(&model_path)?;
    assert!(metadata.len() > 1_000_000, "Model file should be at least 1MB");
    
    println!("Model successfully downloaded to {:?} ({} bytes)", 
             model_path, metadata.len());
    
    Ok(())
} 