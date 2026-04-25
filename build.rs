use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // println!("cargo:rerun-if-changed=configure/");
    println!("cargo:warning=[INFO] build.rs starting ...");
    
    // Get the dir of cargo.toml
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let project_root = Path::new(&manifest_dir);
    
    // build the output path
    let target_dir = project_root.join("target");
    let output_dir = target_dir.join("output");
    if let Err(e) = fs::create_dir_all(&output_dir) {
        println!("cargo:warning=[ERR] Failed to create output directory: {}", e);
    }
    
    // copy the configure dir
    let source_config_dir = project_root.join("configure");
    let target_config_dir = output_dir.join("configure");
    if source_config_dir.exists() {
        println!("cargo:warning=[INFO] Copying configure file from {} to {}", 
                 source_config_dir.display(), target_config_dir.display());
        
        // Remove existing target directory to avoid conflicts
        if target_config_dir.exists() {
            if let Err(e) = fs::remove_dir_all(&target_config_dir) {
                println!("cargo:warning=[ERR] Failed to remove existing target configure dir: {}", e);
            }
        }
        
        // Use cmd.exe /c xcopy command to copy directory
        let cmd_result = Command::new("cmd.exe")
            .args([
                "/C",
                "xcopy",
                source_config_dir.to_str().unwrap(),
                target_config_dir.to_str().unwrap(),
                "/E", "/I", "/Y"
            ])
            .output();
        
        match cmd_result {
            Ok(output) => {
                if output.status.success() {
                    println!("cargo:warning=[INFO] Configure file copied successfully");
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    let stdout_message = String::from_utf8_lossy(&output.stdout);
                    println!("cargo:warning=[ERR] Failed to copy configuration directory: {}", error_message);
                    println!("cargo:warning=[INFO] xcopy output: {}", stdout_message);
                }
            },
            Err(e) => {
                println!("cargo:warning=[ERR] Error executing xcopy: {}", e);
            }
        }
    } else {
        println!("cargo:warning=[ERR] Source configuration directory not found: {}", source_config_dir.display());
    }
    
    // Copy the executable to the output directory
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    println!("cargo:warning=[INFO] build in {}", profile);

    let exe_name = format!("{}.exe", env!("CARGO_PKG_NAME"));
    let target_exe_path = target_dir.join(profile).join(&exe_name);
    // let output_exe_path = output_dir.join(&exe_name);
    
    // Create post-build batch script
    let post_file = "post_build.bat";
    let build_script_path = project_root.join(post_file);
    let script_content = format!(r#"@echo off
REM Build the project
REM cargo build

REM Copy executable to output directory
if exist "{}" (
    echo Copying executable to output directory...
    xcopy "{}" "{}" /I /Y
    echo Executable copied successfully
) else (
    echo Error: Executable not found at {}
)

REM Check if configuration directory was copied successfully
if exist "{}" (
    echo Configuration directory exists in output folder
) else (
    echo Warning: Configuration directory not found in output folder
)
 "#, 
                               target_exe_path.display(),
                               target_exe_path.display(),
                               output_dir.display(),
                               target_exe_path.display(),
                               target_config_dir.display());
    
    if let Err(e) = fs::write(&build_script_path, script_content) {
        println!("cargo:warning=[ERR] Failed to create build script: {}", e);
    } else {
        println!("cargo:warning=[INFO] Run script {} to automatically copy files", post_file);
    }
}