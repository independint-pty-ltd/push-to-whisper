use anyhow::Result;
use std::path::Path;
use push_to_whisper::whisper;

const TEST_AUDIO_FILE: &str = "tests/fixtures/harvard.wav";

// This test is ignored by default as it requires the model file
// Run with: cargo test --test whisper_test -- --ignored
#[tokio::test]
#[ignore]
async fn test_whisper_transcription() -> Result<()> {
    // Skip test if audio file doesn't exist
    if !Path::new(TEST_AUDIO_FILE).exists() {
        println!("Test audio file not found: {}. Skipping test.", TEST_AUDIO_FILE);
        return Ok(());
    }

    // Load the audio file
    let audio_data = load_test_audio()?;
    assert!(!audio_data.is_empty(), "Audio data should not be empty");
    
    // Load the Whisper model with the default model size
    whisper::load_model("medium.en").await?;
    
    // Transcribe the audio using our module's function
    let transcription = whisper::transcribe_audio(&audio_data)?;
    
    // Verify we got some transcription text
    assert!(!transcription.is_empty(), "Transcription should not be empty");
    println!("Transcription: {}", transcription);
    
    // The Harvard sentences are known phrases, so we should check for some keywords
    // The first Harvard sentence is "The birch canoe slid on the smooth planks"
    assert!(
        transcription.to_lowercase().contains("birch") || 
        transcription.to_lowercase().contains("canoe") ||
        transcription.to_lowercase().contains("smooth"),
        "Transcription should contain expected keywords from Harvard sentences"
    );
    
    Ok(())
}

// Helper function to load the test audio file
fn load_test_audio() -> Result<Vec<f32>> {
    use hound::WavReader;
    
    println!("Loading test audio from {}", TEST_AUDIO_FILE);
    let mut reader = WavReader::open(TEST_AUDIO_FILE)?;
    
    let spec = reader.spec();
    println!("Audio format: {} channels, {} Hz", spec.channels, spec.sample_rate);
    
    // Convert to f32 samples
    let mut audio_data: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => {
            match spec.bits_per_sample {
                16 => reader.samples::<i16>()
                    .map(|s| s.map(|s| s as f32 / 32768.0))
                    .collect::<std::result::Result<Vec<f32>, _>>()?,
                24 => reader.samples::<i32>()
                    .map(|s| s.map(|s| s as f32 / 8388608.0))
                    .collect::<std::result::Result<Vec<f32>, _>>()?,
                32 => reader.samples::<i32>()
                    .map(|s| s.map(|s| s as f32 / 2147483648.0))
                    .collect::<std::result::Result<Vec<f32>, _>>()?,
                _ => return Err(anyhow::anyhow!("Unsupported bit depth: {}", spec.bits_per_sample)),
            }
        },
        hound::SampleFormat::Float => {
            reader.samples::<f32>()
                .collect::<std::result::Result<Vec<f32>, _>>()?
        }
    };
    
    // If stereo, convert to mono by averaging channels
    if spec.channels == 2 {
        let mono_data = audio_data.chunks(2)
            .map(|chunk| (chunk[0] + chunk[1]) * 0.5)
            .collect::<Vec<f32>>();
        audio_data = mono_data;
    } else if spec.channels > 2 {
        // Simple approach for multichannel: just average all channels
        let mono_data = audio_data.chunks(spec.channels as usize)
            .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
            .collect::<Vec<f32>>();
        audio_data = mono_data;
    }
    
    // Resample to 16kHz if needed
    if spec.sample_rate != 16000 {
        // Simple downsample by picking every nth sample
        let ratio = spec.sample_rate as f32 / 16000.0;
        let mut resampled_data = Vec::new();
        let mut i = 0.0;
        
        while (i as usize) < audio_data.len() {
            let idx = i as usize;
            if idx < audio_data.len() {
                resampled_data.push(audio_data[idx]);
            }
            i += ratio;
        }
        
        audio_data = resampled_data;
    }
    
    println!("Processed audio: {} samples at 16kHz mono", audio_data.len());
    Ok(audio_data)
} 