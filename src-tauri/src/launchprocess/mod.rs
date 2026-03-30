mod accountRetrieval;
mod classpathBuilder;
mod javaDiscovery;
mod gameSpawning;
mod pathManagement;
mod processDetection;

use std::fs;
use std::path::PathBuf;
use serde_json::Value;

use accountRetrieval::get_current_account_with_log;
use classpathBuilder::{build_classpath, get_asset_index, get_main_class};
use javaDiscovery::find_java_executable;
use gameSpawning::{spawn_minecraft_process, LaunchConfig};
use pathManagement::{emit_log, setup_directories};
use processDetection::is_minecraft_running;

/// Main command for launching Minecraft
/// 
/// Supports two formats:
/// 1. Profile-based launch: `launchprocess(profileName: "MyProfile")`
/// 2. Direct version launch: `launchprocess(version: "1.20.1")`
#[tauri::command]
pub async fn launchprocess(
    app: tauri::AppHandle,
    profileName: Option<String>,
    version: Option<String>,
) -> Result<(), String> {
    // Support both old format (version) and new format (profileName)
    let (actual_version, ram_mb, profile_name_for_log, is_profile) = if let Some(pname) = profileName {
        // New format: profile-based launch
        let profiles = crate::libraryManagement::get_all_profiles().await?;
        let profile = profiles
            .iter()
            .find(|p| p.name == pname)
            .ok_or(format!("Profile '{}' not found", pname))?;

        crate::libraryManagement::update_profile_last_played(pname.clone()).await?;
        (
            profile.base_version.clone(),
            profile.ram_mb,
            Some(pname.clone()),
            true,
        )
    } else if let Some(ver) = version {
        // Old format: direct version launch
        (ver.clone(), 2048, None, false)
    } else {
        return Err("Either profileName or version must be provided".to_string());
    };

    // Emit launch started event
    let profile_info = if let Some(pname) = &profile_name_for_log {
        format!("(Profile: {})", pname)
    } else {
        String::new()
    };
    emit_log(
        &app,
        format!("Starting Minecraft {} {}...", actual_version, profile_info),
    );

    // Check if Minecraft is already running
    if is_minecraft_running()? {
        let msg = "Minecraft is already running";
        emit_log(&app, format!("[ERROR] {}", msg));
        return Err(msg.to_string());
    }

    // Get APPDATA path
    let appdata = std::env::var("APPDATA").map_err(|e| {
        emit_log(&app, format!("[ERROR] Failed to get APPDATA: {}", e));
        e.to_string()
    })?;

    let base_dir = PathBuf::from(&appdata).join(".flint");

    // Setup directory structure
    let dirs = setup_directories(
        base_dir.clone(),
        is_profile,
        profile_name_for_log.as_deref(),
        &actual_version,
    );

    emit_log(&app, format!("Game directory: {}", dirs.mc_dir.display()));

    // Find Java executable
    let java_exe = find_java_executable(&dirs.base_dir, &actual_version).map_err(|e| {
        emit_log(&app, format!("[ERROR] {}", e));
        e
    })?;

    emit_log(&app, format!("Using Java: {}", java_exe));

    // Get current account
    let accounts_path = base_dir.join("accounts.json");
    let username = get_current_account_with_log(&app, &accounts_path)?;

    emit_log(&app, format!("Player: {}", username));

    // Read version JSON
    emit_log(&app, "Building classpath...");

    let version_json_path = base_dir
        .join("versions")
        .join(&actual_version)
        .join(format!("{}.json", &actual_version));

    let json_content = fs::read_to_string(&version_json_path).map_err(|e| {
        emit_log(&app, format!("[ERROR] Failed to read version JSON: {}", e));
        e.to_string()
    })?;

    let version_json: Value = serde_json::from_str(&json_content).map_err(|e| {
        emit_log(&app, format!("[ERROR] Failed to parse version JSON: {}", e));
        e.to_string()
    })?;

    // Extract metadata from version JSON
    let asset_index = get_asset_index(&version_json).map_err(|e| {
        emit_log(&app, format!("[ERROR] {}", e));
        e
    })?;

    let main_class = get_main_class(&version_json).map_err(|e| {
        emit_log(&app, format!("[ERROR] {}", e));
        e
    })?;

    // Build classpath
    let main_jar = base_dir
        .join("versions")
        .join(&actual_version)
        .join(format!("{}.jar", &actual_version));

    let classpath = build_classpath(&version_json, &dirs.libraries_dir, &main_jar).map_err(|e| {
        emit_log(&app, format!("[ERROR] {}", e));
        e
    })?;

    emit_log(&app, format!("Loaded {} libraries", count_jars(&classpath)));

    // Prepare native library path
    let java_library_path = format!("-Djava.library.path={}", dirs.natives_dir.display());

    // Launch the game
    let config = LaunchConfig {
        java_exe,
        main_class,
        classpath,
        java_library_path,
        version: actual_version.to_string(),
        username,
        asset_index,
        game_dir: dirs.mc_dir,
        assets_dir: dirs.assets_dir,
        ram_mb,
    };

    spawn_minecraft_process(&app, config).await?;

    Ok(())
}

/// Helper function to count JARs in classpath
fn count_jars(classpath: &str) -> usize {
    classpath.split(';').count()
}
