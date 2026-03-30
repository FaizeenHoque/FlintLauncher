use std::process::Command;

/// Checks if Minecraft (Java) is already running
/// 
/// Returns `true` if java.exe is currently running, `false` otherwise
#[allow(dead_code)]
pub fn is_minecraft_running() -> Result<bool, String> {
    let output = Command::new("tasklist")
        .output()
        .map_err(|e| e.to_string())?;
    
    let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    Ok(stdout.to_lowercase().contains("java.exe"))
}
