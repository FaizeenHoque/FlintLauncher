use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::fs;
use serde_json::Value;
use tauri::Emitter;

#[allow(dead_code)]
fn is_minecraft_running() -> Result<bool, String> {
    // Check if java.exe is running
    let output = Command::new("tasklist")
        .output()
        .map_err(|e| e.to_string())?;
    
    let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    Ok(stdout.to_lowercase().contains("java.exe"))
}

fn find_java_executable(flint_dir: &PathBuf, version: &str) -> Result<String, String> {
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

    // Try bundled Java installations
    let bundled_java = flint_dir.join("runtime").join("java-runtime-gamma").join("bin").join("java.exe");
    if bundled_java.exists() {
        return Ok(bundled_java.to_string_lossy().to_string());
    }

    let bundled_java = flint_dir.join("runtime").join("java-runtime-alpha").join("bin").join("java.exe");
    if bundled_java.exists() {
        return Ok(bundled_java.to_string_lossy().to_string());
    }

    let bundled_java = flint_dir.join("runtime").join("jre-legacy").join("bin").join("java.exe");
    if bundled_java.exists() {
        return Ok(bundled_java.to_string_lossy().to_string());
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

#[tauri::command]
pub async fn launchprocess(app: tauri::AppHandle, profile_name: Option<String>, version: Option<String>) -> Result<(), String> {
    // Support both old format (version) and new format (profile_name)
    let (actual_version, ram_mb, profile_name_for_log, is_profile) = if let Some(pname) = profile_name {
        // New format: profile-based launch
        let profiles = crate::library::get_all_profiles().await?;
        let profile = profiles.iter()
            .find(|p| p.name == pname)
            .ok_or(format!("Profile '{}' not found", pname))?;
        
        crate::library::update_profile_last_played(pname.clone()).await?;
        (profile.base_version.clone(), profile.ram_mb, Some(pname.clone()), true)
    } else if let Some(ver) = version {
        // Old format: direct version launch
        (ver, 2048, None, false)
    } else {
        return Err("Either profile_name or version must be provided".to_string());
    };

    // Emit launch started event
    let _ = app.emit("launch-log", serde_json::json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": if let Some(pname) = &profile_name_for_log {
            format!("Starting Minecraft {} (Profile: {})...", actual_version, pname)
        } else {
            format!("Starting Minecraft {}...", actual_version)
        }
    }));

    // Check if Minecraft is already running
    if is_minecraft_running()? {
        let msg = "Minecraft is already running";
        let _ = app.emit("launch-log", serde_json::json!({
            "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
            "message": format!("[ERROR] {}", msg)
        }));
        return Err(msg.to_string());
    }
    
    // Get APPDATA path and determine game directory based on profile or vanilla launch
    let appdata = std::env::var("APPDATA").map_err(|e| {
        let _ = app.emit("launch-log", serde_json::json!({
            "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
            "message": format!("[ERROR] Failed to get APPDATA: {}", e)
        }));
        e.to_string()
    })?;
    
    // Base directory for shared game files (versions, libraries, assets)
    let base_dir = PathBuf::from(&appdata).join(".flint");
    
    // Game directory for saves/config (profile-specific or vanilla)
    let mc_dir = if is_profile && profile_name_for_log.is_some() {
        // Profile-based launch: use isolated directory for saves/config
        let profile_name = profile_name_for_log.as_ref().unwrap();
        base_dir.join("instances").join(profile_name)
    } else {
        // Vanilla launch: use default .flint directory for saves/config
        base_dir.clone()
    };
    
    let _ = app.emit("launch-log", serde_json::json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": format!("Game directory: {}", mc_dir.display())
    }));
    
    // Find Java executable
    let java_exe = find_java_executable(&mc_dir, &actual_version).map_err(|e| {
        let _ = app.emit("launch-log", serde_json::json!({
            "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
            "message": format!("[ERROR] {}", e)
        }));
        e
    })?;

    let _ = app.emit("launch-log", serde_json::json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": format!("Using Java: {}", java_exe)
    }));
    
    // Read current account from accounts.json (shared across all profiles)
    let accounts_path = base_dir.join("accounts.json");
    let username = if accounts_path.exists() {
        let raw = fs::read_to_string(&accounts_path).map_err(|e| {
            let _ = app.emit("launch-log", serde_json::json!({
                "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
                "message": format!("[ERROR] Failed to read accounts: {}", e)
            }));
            e.to_string()
        })?;
        let data: Value = serde_json::from_str(&raw).map_err(|e| {
            let _ = app.emit("launch-log", serde_json::json!({
                "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
                "message": format!("[ERROR] Failed to parse accounts: {}", e)
            }));
            e.to_string()
        })?;
        
        // Handle migration from old array format to new object format
        let data = if data.is_array() {
            serde_json::json!({"accounts": data, "current": null})
        } else {
            data
        };
        
        data["current"]
            .as_str()
            .ok_or_else(|| {
                let msg = "No account selected";
                let _ = app.emit("launch-log", serde_json::json!({
                    "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
                    "message": format!("[ERROR] {}", msg)
                }));
                msg.to_string()
            })?
            .to_string()
    } else {
        let msg = "No accounts found";
        let _ = app.emit("launch-log", serde_json::json!({
            "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
            "message": format!("[ERROR] {}", msg)
        }));
        return Err(msg.to_string());
    };
    
    let _ = app.emit("launch-log", serde_json::json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": format!("Player: {}", username)
    }));
    
    // Read version JSON (from shared base directory)
    let version_json_path = base_dir.join("versions").join(&actual_version).join(format!("{}.json", &actual_version));
    let json_content = fs::read_to_string(&version_json_path).map_err(|e| {
        let _ = app.emit("launch-log", serde_json::json!({
            "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
            "message": format!("[ERROR] Failed to read version JSON: {}", e)
        }));
        e.to_string()
    })?;
    let version_json: Value = serde_json::from_str(&json_content).map_err(|e| {
        let _ = app.emit("launch-log", serde_json::json!({
            "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
            "message": format!("[ERROR] Failed to parse version JSON: {}", e)
        }));
        e.to_string()
    })?;
    
    let _ = app.emit("launch-log", serde_json::json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": "Building classpath..."
    }));
    
    // Extract asset index and main class
    let asset_index = version_json["assetIndex"]["id"]
        .as_str()
        .ok_or_else(|| {
            let _ = app.emit("launch-log", serde_json::json!({
                "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
                "message": "[ERROR] Missing assetIndex.id"
            }));
            "Missing assetIndex.id".to_string()
        })?;
    let main_class = version_json["mainClass"]
        .as_str()
        .ok_or_else(|| {
            let _ = app.emit("launch-log", serde_json::json!({
                "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
                "message": "[ERROR] Missing mainClass"
            }));
            "Missing mainClass".to_string()
        })?;
    
    // Build classpath (from shared base directory)
    let assets_dir = base_dir.join("assets");
    let libraries_dir = base_dir.join("libraries");
    let mut jars = Vec::new();
    
    if let Some(libs) = version_json["libraries"].as_array() {
        for lib in libs {
            if let Some(artifact) = lib["downloads"]["artifact"].as_object() {
                if let Some(path_str) = artifact.get("path").and_then(|p| p.as_str()) {
                    let jar_path = libraries_dir.join(path_str);
                    if jar_path.exists() {
                        jars.push(jar_path);
                    }
                }
            }
        }
    }
    
    // Add main jar (from shared base directory)
    let main_jar = base_dir.join("versions").join(&actual_version).join(format!("{}.jar", &actual_version));
    jars.push(main_jar);
    
    let _ = app.emit("launch-log", serde_json::json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": format!("Loaded {} libraries", jars.len())
    }));
    
    // Build classpath string (semicolon-separated for Windows)
    let classpath = jars
        .iter()
        .filter_map(|p| p.to_str())
        .collect::<Vec<_>>()
        .join(";");
    
    // Prepare native library path
    let natives_path = base_dir.join("versions").join(&actual_version).join("natives");
    let java_library_path = format!("-Djava.library.path={}", natives_path.display());
    
    let _ = app.emit("launch-log", serde_json::json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": "Launching game..."
    }));
    
    // Spawn Java process with visible console window
    let mut cmd = Command::new(&java_exe);
    let max_ram = format!("-Xmx{}M", ram_mb);
    let min_ram = format!("-Xms{}M", ram_mb / 2);
    cmd.arg(&max_ram)
        .arg(&min_ram)
        .arg(&java_library_path)
        .arg(format!("-cp"))
        .arg(&classpath)
        .arg(main_class)
        .arg("--version")
        .arg(&actual_version)
        .arg("--gameDir")
        .arg(mc_dir.to_str().ok_or("Invalid path")?)
        .arg("--assetsDir")
        .arg(assets_dir.to_str().ok_or("Invalid path")?)
        .arg("--assetIndex")
        .arg(asset_index)
        .arg("--uuid")
        .arg("00000000-0000-0000-0000-000000000000")
        .arg("--accessToken")
        .arg("0")
        .arg("--enable-native-access=ALL-UNNAMED")
        .arg("--username")
        .arg(&username)
        .arg("--userType")
        .arg("legacy")
        .arg("--versionType")
        .arg("release")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    
    let mut child = cmd.spawn().map_err(|e| {
        let msg = format!("Failed to launch game: {}", e);
        let _ = app.emit("launch-log", serde_json::json!({
            "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
            "message": format!("[ERROR] {}", msg)
        }));
        msg
    })?;
    
    let _ = app.emit("launch-log", serde_json::json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": format!("Game launched with PID: {} - Terminal window will stay open", child.id())
    }));
    
    Ok(())
}