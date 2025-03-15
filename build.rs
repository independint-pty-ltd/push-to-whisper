use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    
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

        // Write the resource file
        let resource_content = r#"#include <windows.h>

normal-icon ICON "icons/normal_icon.ico"
recording-icon ICON "icons/recording_icon.ico"
"#;
        fs::write("resources.rc", resource_content).expect("Failed to write resource file");

        // Compile the resource file on Windows
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
} 