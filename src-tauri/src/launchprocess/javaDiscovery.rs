use std::process::Command;
use std::path::PathBuf;
use std::fs;
use serde_json::Value;

/// Finds the Java executable to use for launching Minecraft
/// 
/// Tries these locations in order:
/// 1. Java path from version metadata (flint_meta.json)
/// 2. Bundled Java runtime (gamma, alpha, legacy)
/// 3. System Java (from PATH)
pub fn find_java_executable(flint_dir: &PathBuf, version: &str) -> Result<String, String> {
    // First, try to get Java path from version metadata
    let meta_path = flint_dir.join("versions").join(version).join("flint_meta.json");
    if meta_path.exists() {
        if let Ok(content) = fs::read_to_string(&meta_path) {
            if let Ok(meta) = serde_json::from_str::<Value>(&content) {
                if let Some(java_path) = meta["javaExe"].as_str() {
                    if PathBuf::from(java_path).exists() {
                        return Ok(java_path.to_string());
                    }
                }
            }
        }
    }

    // Try bundled Java installations (in priority order)
    let bundled_runtimes = vec![
        ("java-runtime-gamma", "Gamma runtime"),
        ("java-runtime-alpha", "Alpha runtime"),
        ("jre-legacy", "Legacy JRE"),
    ];

    for (runtime_name, _label) in bundled_runtimes {
        let bundled_java = flint_dir.join("runtime").join(runtime_name).join("bin").join("java.exe");
        if bundled_java.exists() {
            return Ok(bundled_java.to_string_lossy().to_string());
        }
    }

    // Fall back to system Java (check PATH)
    match Command::new("where").arg("java.exe").output() {
        Ok(output) => {
            let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
            let java_path = stdout.lines().next().ok_or("Java not found in PATH")?;
            Ok(java_path.trim().to_string())
        }
        Err(_) => {
            Err("Java executable not found. Please install Java or ensure it is in your PATH.".to_string())
        }
    }
}
