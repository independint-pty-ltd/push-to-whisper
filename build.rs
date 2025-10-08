use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=CUDA_ARCH");
    println!("cargo:rerun-if-env-changed=CUDA_MULTI_ARCH");
    
    // Check if we should build for multiple architectures
    let multi_arch = match env::var("CUDA_MULTI_ARCH") {
        Ok(val) => match val.as_str() {
            "1" | "true" | "TRUE" | "True" | "yes" | "YES" | "Yes" => true,
            _ => false,
        },
        Err(_) => {
            // Default to single architecture to optimize binary size
            // This is set to target only RTX 4090 (compute capability 8.9)
            false
        }
    };
    
    // Get CUDA architecture from environment or use a single default
    // Common architecture values:
    // - 52: Maxwell (GTX 9xx)
    // - 60/61: Pascal (GTX 10xx) 
    // - 70: Volta (Titan V)
    // - 75: Turing (RTX 20xx, GTX 16xx)
    // - 86: Ampere (RTX 30xx)
    // - 89: Ada Lovelace (RTX 40xx)
    let cuda_arch = if let Ok(arch_var) = env::var("CUDA_ARCH") {
        // User specified architecture
        if (arch_var.contains(';') || arch_var.contains(',')) && !multi_arch {
            println!("cargo:warning=WARNING: Multiple architectures detected in CUDA_ARCH, but CUDA_MULTI_ARCH is not enabled.");
            println!("cargo:warning=  Using only the first architecture to keep binary size smaller.");
            println!("cargo:warning=  Set CUDA_MULTI_ARCH=1 if you want to build for multiple architectures.");
            
            // Get the first architecture, handling both comma and semicolon separators
            if arch_var.contains(';') {
                arch_var.split(';').next().unwrap_or("89").to_string()
            } else {
                arch_var.split(',').next().unwrap_or("89").to_string()
            }
        } else {
            arch_var
        }
    } else {
        // Default to Ada Lovelace architecture (RTX 40xx)
        String::from("89")
    };
    
    // Process the architecture string to handle different formats
    // Convert any architecture strings that look like "compute_XX,sm_XX" to just "XX"
    // and normalize separators to semicolons
    let normalized_arch = cuda_arch
        .replace(',', ";") // First convert all commas to semicolons
        .split(';')
        .map(|a| {
            let a = a.trim();
            if a.contains("compute_") && a.contains("sm_") {
                // Extract just the number from patterns like "compute_89,sm_89"
                if let Some(num) = a.split("compute_").nth(1).and_then(|s| s.split(',').next()) {
                    num.to_string()
                } else {
                    a.to_string()
                }
            } else if a.contains("compute_") {
                // Extract just the number from "compute_89"
                if let Some(num) = a.split("compute_").nth(1) {
                    num.to_string()
                } else {
                    a.to_string()
                }
            } else if a.contains("sm_") {
                // Extract just the number from "sm_89"
                if let Some(num) = a.split("sm_").nth(1) {
                    num.to_string()
                } else {
                    a.to_string()
                }
            } else if a.contains("ptx") {
                // Keep track of PTX
                "ptx".to_string()
            } else {
                a.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(";");
    
    // Check if multiple architectures are being built
    let is_multi_arch = normalized_arch.contains(';');
    let architectures: Vec<String> = normalized_arch.split(';').map(|s| s.to_string()).collect();
    
    // Check for PTX output
    let has_ptx = architectures.iter().any(|a| a == "ptx" || a.contains("ptx"));
    
    if is_multi_arch {
        println!("cargo:warning=Building for MULTIPLE CUDA architectures: {} architectures", architectures.len());
        
        // Print each architecture with clear labels
        println!("cargo:warning=Target architectures:");
        for arch in &architectures {
            if arch == "ptx" || arch.contains("ptx") {
                println!("cargo:warning=  - PTX (JIT compilation for future GPUs)");
            } else {
                println!("cargo:warning=  - SM_{} ({} series GPUs)", 
                    arch,
                    match arch.as_str() {
                        "52" => "Maxwell/GTX 900",
                        "60" | "61" => "Pascal/GTX 1000",
                        "70" => "Volta/Titan V",
                        "75" => "Turing/RTX 2000",
                        "86" | "87" => "Ampere/RTX 3000",
                        "89" => "Ada Lovelace/RTX 4000",
                        _ => "Unknown",
                    }
                );
            }
        }
        
        println!("cargo:warning=Binary size will increase significantly with multiple architectures!");
    } else {
        // Single architecture build
        let arch = &architectures[0];
        if arch == "ptx" || arch.contains("ptx") {
            println!("cargo:warning=Building for PTX JIT compilation (future GPU compatibility)");
        } else {
            println!("cargo:warning=Building for SINGLE CUDA architecture: SM_{} ({} series GPUs)", 
                arch,
                match arch.as_str() {
                    "52" => "Maxwell/GTX 900",
                    "60" | "61" => "Pascal/GTX 1000",
                    "70" => "Volta/Titan V",
                    "75" => "Turing/RTX 2000",
                    "86" | "87" => "Ampere/RTX 3000",
                    "89" => "Ada Lovelace/RTX 4000",
                    _ => "Unknown",
                }
            );
        }
        
        println!("cargo:warning=Optimized binary size for specific GPU generation.");
    }
    
    // Include note about PTX if present
    if has_ptx {
        println!("cargo:warning=PTX output enables JIT compilation for future GPU architectures,");
        println!("cargo:warning=but increases binary size and compilation time.");
    }
    
    println!("cargo:warning=  To target different architectures, set environment variables:");
    println!("cargo:warning=    - CUDA_ARCH=X (where X is the architecture number, e.g. 75 for RTX 2000)");
    println!("cargo:warning=    - CUDA_MULTI_ARCH=1 (to enable building for multiple architectures)");
    
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
            
            // Set NVCC flags for specified architecture(s)
            let arch_flags: Vec<String> = architectures.iter()
                .filter(|a| *a != "ptx" && !a.contains("ptx"))
                .map(|arch| format!("-gencode=arch=compute_{0},code=sm_{0}", arch))
                .collect();
            
            // Add PTX generation flag if needed
            let nvcc_flags = if has_ptx {
                let mut flags = arch_flags.clone();
                // Generate PTX code for the highest architecture
                if let Some(highest_arch) = architectures.iter()
                    .filter(|a| *a != "ptx" && !a.contains("ptx"))
                    .max_by(|a, b| {
                        a.parse::<i32>().unwrap_or(0).cmp(&b.parse::<i32>().unwrap_or(0))
                    }) 
                {
                    flags.push(format!("-gencode=arch=compute_{0},code=compute_{0}", highest_arch));
                    println!("cargo:warning=Including PTX JIT for compute_{} (future GPU compatibility)", highest_arch);
                }
                flags.join(" ")
            } else {
                arch_flags.join(" ")
            };
            
            println!("cargo:warning=Using NVCC flags: {}", nvcc_flags);
            println!("cargo:rustc-env=NVCC_FLAGS={}", nvcc_flags);
        } else {
            println!("cargo:warning=CUDA_PATH is set but directory does not exist: {}", cuda_path.display());
        }
    } else {
        println!("cargo:warning=CUDA_PATH not set, will attempt to use system CUDA if available");
        
        // Check common CUDA installation paths (add CUDA 13.x)
        let common_paths = [
            // CUDA 13.x
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v13.1",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v13.0",
            // CUDA 12.x
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.9",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.8",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.7",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.6",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.5",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.4",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.3",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.2",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.1",
            "C:/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v12.0",
            // CUDA 11.x
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
                
                // Set NVCC flags for specified architecture(s)
                let arch_flags: Vec<String> = architectures.iter()
                    .filter(|a| *a != "ptx" && !a.contains("ptx"))
                    .map(|arch| format!("-gencode=arch=compute_{0},code=sm_{0}", arch))
                    .collect();
                
                // Add PTX generation flag if needed
                let nvcc_flags = if has_ptx {
                    let mut flags = arch_flags.clone();
                    // Generate PTX code for the highest architecture
                    if let Some(highest_arch) = architectures.iter()
                        .filter(|a| *a != "ptx" && !a.contains("ptx"))
                        .max_by(|a, b| {
                            a.parse::<i32>().unwrap_or(0).cmp(&b.parse::<i32>().unwrap_or(0))
                        }) 
                    {
                        flags.push(format!("-gencode=arch=compute_{0},code=compute_{0}", highest_arch));
                        println!("cargo:warning=Including PTX JIT for compute_{} (future GPU compatibility)", highest_arch);
                    }
                    flags.join(" ")
                } else {
                    arch_flags.join(" ")
                };
                
                println!("cargo:warning=Using NVCC flags: {}", nvcc_flags);
                println!("cargo:rustc-env=NVCC_FLAGS={}", nvcc_flags);
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