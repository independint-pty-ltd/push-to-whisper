use anyhow::{Result, Context};
use log::{info, warn, error};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

const MODEL_URLS: &[(&str, &str)] = &[
    ("tiny.en", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.en.bin"),
    ("base.en", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin"),
    ("small.en", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.en.bin"),
    ("medium.en", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.en.bin"),
    ("large", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin"),
];

const MODEL_BASE_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-";

pub fn get_model_path(model_size: &str) -> PathBuf {
    let exe_dir = std::env::current_exe()
        .expect("Failed to get executable path")
        .parent()
        .expect("Failed to get executable directory")
        .to_path_buf();
    
    exe_dir.join("models").join(format!("{}.bin", model_size))
}

pub async fn ensure_model_exists(model_size: &str) -> Result<PathBuf> {
    let model_path = get_model_path(model_size);
    
    if !model_path.exists() {
        info!("Model not found at {:?}, downloading...", model_path);
        
        // Create the models directory if it doesn't exist
        if let Some(parent) = model_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create models directory at {:?}", parent))?;
        }
        
        download_model(model_size, &model_path).await?;
    } else {
        info!("Using existing model at {:?}", model_path);
    }
    
    Ok(model_path)
}

async fn download_model(model_size: &str, path: &PathBuf) -> Result<()> {
    let url = format!("{}{}.bin", MODEL_BASE_URL, model_size);
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let total_size = response.content_length().unwrap_or(0);
    
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    
    let mut file = std::fs::File::create(path)?;
    let mut stream = response.bytes_stream();
    let mut downloaded: u64 = 0;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }
    
    pb.finish_with_message("Download complete");
    Ok(())
}

pub fn list_available_models() -> Vec<String> {
    MODEL_URLS.iter().map(|(size, _)| size.to_string()).collect()
} 