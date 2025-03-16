use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let version = if args.len() > 1 { &args[1] } else { "0.1.0" };
    
    println!("Creating release package for Push-to-Whisper v{}", version);
    
    // Get the project root directory
    let project_root = get_project_root()?;
    println!("Project root: {}", project_root.display());
    
    // Define paths
    let release_dir = project_root.join("release");
    let target_dir = project_root.join("target").join("release");
    let temp_dir = release_dir.join("temp_package");
    let zip_file_name = format!("push-to-whisper-v{}.zip", version);
    let zip_file_path = release_dir.join(&zip_file_name);
    
    // Create release directory if it doesn't exist
    if !release_dir.exists() {
        println!("Creating release directory: {}", release_dir.display());
        fs::create_dir_all(&release_dir)?;
    }
    
    // Check if the executable exists
    let exe_name = if cfg!(windows) {
        "push-to-whisper.exe"
    } else {
        "push-to-whisper"
    };
    let exe_path = target_dir.join(exe_name);
    
    if !exe_path.exists() {
        eprintln!("Error: Executable not found at {}", exe_path.display());
        eprintln!("Please build the project first with 'cargo build --release'");
        return Err(io::Error::new(io::ErrorKind::NotFound, "Executable not found"));
    }
    
    // Check if README.txt exists
    let readme_path = release_dir.join("README.txt");
    if !readme_path.exists() {
        eprintln!("Error: README.txt not found at {}", readme_path.display());
        return Err(io::Error::new(io::ErrorKind::NotFound, "README.txt not found"));
    }
    
    // Create a temporary directory for packaging
    if temp_dir.exists() {
        println!("Cleaning up existing temporary directory");
        fs::remove_dir_all(&temp_dir)?;
    }
    println!("Creating temporary directory: {}", temp_dir.display());
    fs::create_dir_all(&temp_dir)?;
    
    // Copy files to the temporary directory
    println!("Copying executable to temporary directory");
    fs::copy(&exe_path, temp_dir.join(exe_name))?;
    
    println!("Copying README.txt to temporary directory");
    fs::copy(&readme_path, temp_dir.join("README.txt"))?;
    
    // Create the zip file
    println!("Creating release package: {}", zip_file_path.display());
    if zip_file_path.exists() {
        println!("Removing existing zip file");
        fs::remove_file(&zip_file_path)?;
    }
    
    // Create the zip file using the appropriate command for the platform
    create_zip(&temp_dir, &zip_file_path)?;
    
    // Clean up the temporary directory
    println!("Cleaning up temporary directory");
    fs::remove_dir_all(&temp_dir)?;
    
    // Check if the zip file was created successfully
    if zip_file_path.exists() {
        let file_size = fs::metadata(&zip_file_path)?.len();
        let file_size_mb = file_size as f64 / 1_048_576.0;
        println!("Release package created successfully: {} ({:.2} MB)", zip_file_path.display(), file_size_mb);
        
        // List the contents of the zip file
        println!("Package contents:");
        list_zip_contents(&zip_file_path)?;
    } else {
        eprintln!("Error: Failed to create release package");
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to create zip file"));
    }
    
    println!("\nTo create a release with a different version number:");
    println!("cargo run --release --bin create_release <version>");
    
    Ok(())
}

fn get_project_root() -> io::Result<PathBuf> {
    // Try to find the project root by looking for Cargo.toml
    let mut current_dir = env::current_dir()?;
    
    loop {
        if current_dir.join("Cargo.toml").exists() {
            return Ok(current_dir);
        }
        
        if !current_dir.pop() {
            break;
        }
    }
    
    // If we couldn't find it, just use the current directory
    env::current_dir()
}

fn create_zip(source_dir: &Path, zip_file: &Path) -> io::Result<()> {
    if cfg!(windows) {
        // Use PowerShell on Windows
        let status = Command::new("powershell")
            .arg("-Command")
            .arg(&format!("Compress-Archive -Path \"{}\\*\" -DestinationPath \"{}\"", 
                source_dir.display(), zip_file.display()))
            .status()?;
        
        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to create zip file with PowerShell"));
        }
    } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        // Use zip command on macOS and Linux
        let status = Command::new("zip")
            .arg("-r")
            .arg(zip_file)
            .arg(".")
            .current_dir(source_dir)
            .status()?;
        
        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to create zip file with zip command"));
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Unsupported platform"));
    }
    
    Ok(())
}

fn list_zip_contents(zip_file: &Path) -> io::Result<()> {
    if cfg!(windows) {
        // Use PowerShell on Windows
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(&format!("Get-ChildItem -Path \"{}\" | Select-Object -ExpandProperty Name", 
                zip_file.display()))
            .output()?;
        
        if output.status.success() {
            let contents = String::from_utf8_lossy(&output.stdout);
            for line in contents.lines() {
                println!("- {}", line);
            }
        } else {
            eprintln!("Failed to list zip contents");
        }
    } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        // Use unzip -l on macOS and Linux
        let output = Command::new("unzip")
            .arg("-l")
            .arg(zip_file)
            .output()?;
        
        if output.status.success() {
            let contents = String::from_utf8_lossy(&output.stdout);
            let mut lines = contents.lines().skip(3); // Skip header lines
            
            while let Some(line) = lines.next() {
                if line.trim().is_empty() || line.contains("--------") {
                    continue;
                }
                
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    println!("- {}", parts[3..].join(" "));
                }
            }
        } else {
            eprintln!("Failed to list zip contents");
        }
    }
    
    Ok(())
} 