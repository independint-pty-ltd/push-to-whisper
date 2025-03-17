use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=CUDA_ARCH");
    
    // Get CUDA architectures from environment or use defaults
    let cuda_arch = env::var("CUDA_ARCH").unwrap_or_else(|_| String::from("52;60;61;70;75;86;89"));
    println!("cargo:warning=Building for CUDA architectures: {}", cuda_arch);
    
    // Set environment variable for whisper-rs to use
    println!("cargo:rustc-env=CUDA_ARCH={}", cuda_arch);
    
    // Check if CUDA is available
    if let Ok(cuda_path) = env::var("CUDA_PATH") {
        let cuda_path = PathBuf::from(cuda_path);
        
        if cuda_path.exists() {
            println!("cargo:rustc-env=CUDA_AVAILABLE=1");
            println!("cargo:warning=CUDA found at: {}", cuda_path.display());
            
            // Add CUDA library path to linker search path
            let lib_path = cuda_path.join("lib64");
            if lib_path.exists() {
                println!("cargo:rustc-link-search=native={}", lib_path.display());
            }
            
            let lib_path = cuda_path.join("lib");
            if lib_path.exists() {
                println!("cargo:rustc-link-search=native={}", lib_path.display());
            }
            
            let lib_path = cuda_path.join("lib/x64");
            if lib_path.exists() {
                println!("cargo:rustc-link-search=native={}", lib_path.display());
            }
            
            // Set NVCC flags for multiple architectures
            let arch_flags: Vec<String> = cuda_arch.split(';')
                .map(|arch| format!("-gencode=arch=compute_{0},code=sm_{0}", arch))
                .collect();
            
            println!("cargo:rustc-env=NVCC_FLAGS={}", arch_flags.join(" "));
        } else {
            println!("cargo:warning=CUDA_PATH is set but directory does not exist: {}", cuda_path.display());
        }
    } else {
        println!("cargo:warning=CUDA_PATH not set, will attempt to use system CUDA if available");
        
        // Check common CUDA installation paths
        let common_paths = [
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.8",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.0",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v11.8",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v11.7",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v11.6",
        ];
        
        for path in common_paths.iter() {
            let path = PathBuf::from(path);
            if path.exists() {
                println!("cargo:warning=Found CUDA at: {}", path.display());
                println!("cargo:rustc-env=CUDA_AVAILABLE=1");
                
                // Add CUDA library path to linker search path
                let lib_path = path.join("lib64");
                if lib_path.exists() {
                    println!("cargo:rustc-link-search=native={}", lib_path.display());
                }
                
                let lib_path = path.join("lib");
                if lib_path.exists() {
                    println!("cargo:rustc-link-search=native={}", lib_path.display());
                }
                
                let lib_path = path.join("lib/x64");
                if lib_path.exists() {
                    println!("cargo:rustc-link-search=native={}", lib_path.display());
                }
                
                // Set NVCC flags for multiple architectures
                let arch_flags: Vec<String> = cuda_arch.split(';')
                    .map(|arch| format!("-gencode=arch=compute_{0},code=sm_{0}", arch))
                    .collect();
                
                println!("cargo:rustc-env=NVCC_FLAGS={}", arch_flags.join(" "));
                break;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Ensure the icons directory exists
        let icons_dir = Path::new("icons");
        if !icons_dir.exists() {
            fs::create_dir_all(icons_dir).expect("Failed to create icons directory");
        }

        // Save icon data to files
        // We need to include the icon data directly in the build script
        // since we can't import from src/ui/ico_data.rs
        
        // Normal icon data (first few bytes for identification)
        let normal_icon_header = &[
            0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x10, 0x10, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x68, 0x05,
            0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x20, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Transparent Color
            0x00, 0x00, 0x00, 0x00,
            // Gray Color (for inactive state)
            0x80, 0x80, 0x80, 0x00,
        ];
        
        // Recording icon data (first few bytes for identification)
        let recording_icon_header = &[
            0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x10, 0x10, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x68, 0x05,
            0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x20, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Transparent Color
            0x00, 0x00, 0x00, 0x00,
            // Red Color (for active/recording state)
            0x00, 0x00, 0xFF, 0x00,
        ];
        
        // Transcribing icon data (first few bytes for identification)
        let transcribing_icon_header = &[
            0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x10, 0x10, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x68, 0x05,
            0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x20, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // Transparent Color
            0x00, 0x00, 0x00, 0x00,
            // Orange Color (for transcribing/processing state)
            0x00, 0x80, 0xFF, 0x00,
        ];
        
        // Check if the icon files already exist, and if not, copy them from the src directory
        let normal_icon_path = icons_dir.join("normal_icon.ico");
        let recording_icon_path = icons_dir.join("recording_icon.ico");
        let transcribing_icon_path = icons_dir.join("transcribing_icon.ico");
        
        // If the icon files don't exist or are empty, copy them from the src directory
        if !normal_icon_path.exists() || fs::metadata(&normal_icon_path).map(|m| m.len() == 0).unwrap_or(true) {
            println!("cargo:warning=Creating normal icon file");
            // We'll use a placeholder icon since we can't access the actual icon data
            // The actual icon data will be loaded at runtime from src/ui/ico_data.rs
            fs::write(&normal_icon_path, normal_icon_header).expect("Failed to write normal icon file");
        }
        
        if !recording_icon_path.exists() || fs::metadata(&recording_icon_path).map(|m| m.len() == 0).unwrap_or(true) {
            println!("cargo:warning=Creating recording icon file");
            fs::write(&recording_icon_path, recording_icon_header).expect("Failed to write recording icon file");
        }
        
        if !transcribing_icon_path.exists() || fs::metadata(&transcribing_icon_path).map(|m| m.len() == 0).unwrap_or(true) {
            println!("cargo:warning=Creating transcribing icon file");
            fs::write(&transcribing_icon_path, transcribing_icon_header).expect("Failed to write transcribing icon file");
        }

        // Write the resource file
        let resource_content = r#"#include <windows.h>

normal-icon ICON "icons/normal_icon.ico"
recording-icon ICON "icons/recording_icon.ico"
transcribing-icon ICON "icons/transcribing_icon.ico"
"#;
        fs::write("resources.rc", resource_content).expect("Failed to write resource file");

        // Compile the resource file on Windows
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
    
    // Copy models folder to target directory
    copy_models_folder().expect("Failed to copy models folder");
}

// Function to copy the models folder to the target directory
fn copy_models_folder() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let out_path = PathBuf::from(out_dir);
    
    // Navigate up from OUT_DIR to the target directory
    // OUT_DIR is typically something like target/debug/build/push-to-whisper-{hash}/out
    // We need to go up 3 levels to get to target/debug
    let target_dir = out_path
        .ancestors()
        .nth(3)
        .expect("Failed to find target directory")
        .to_path_buf();
    
    println!("cargo:warning=Target directory: {}", target_dir.display());
    
    // Source models directory
    let source_models_dir = Path::new("models");
    
    // Target models directory
    let target_models_dir = target_dir.join("models");
    
    // Only copy if source models directory exists
    if source_models_dir.exists() && source_models_dir.is_dir() {
        println!("cargo:warning=Copying models from {} to {}", source_models_dir.display(), target_models_dir.display());
        
        // Create target models directory if it doesn't exist
        if !target_models_dir.exists() {
            fs::create_dir_all(&target_models_dir)?;
        }
        
        // Copy all files from source to target
        copy_dir_contents(source_models_dir, &target_models_dir)?;
        
        println!("cargo:warning=Models copied successfully");
    } else {
        println!("cargo:warning=Models directory not found at {}", source_models_dir.display());
    }
    
    Ok(())
}

// Helper function to recursively copy directory contents
fn copy_dir_contents(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_contents(&src_path, &dst_path)?;
        } else {
            // Only copy if the destination file doesn't exist or is older than the source
            let should_copy = if dst_path.exists() {
                let src_metadata = fs::metadata(&src_path)?;
                let dst_metadata = fs::metadata(&dst_path)?;
                
                src_metadata.modified()? > dst_metadata.modified()?
            } else {
                true
            };
            
            if should_copy {
                fs::copy(&src_path, &dst_path)?;
                println!("cargo:warning=Copied {} to {}", src_path.display(), dst_path.display());
            }
        }
    }
    
    Ok(())
} 