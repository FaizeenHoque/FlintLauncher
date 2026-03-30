use std::path::PathBuf;
use serde_json::json;
use tauri::Emitter;

/// Represents the directory structure for Minecraft files
pub struct GameDirectories {
    /// Base directory for shared files (versions, libraries, assets)
    pub base_dir: PathBuf,
    /// Game directory for saves/config (profile-specific or vanilla)
    pub mc_dir: PathBuf,
    /// Assets directory
    pub assets_dir: PathBuf,
    /// Libraries directory
    pub libraries_dir: PathBuf,
    /// Natives directory for version
    pub natives_dir: PathBuf,
}

/// Builds the game directory structure
/// 
/// For profile-based launches: creates profile-specific directories
/// For vanilla launches: uses the base .flint directory
pub fn setup_directories(
    base_dir: PathBuf,
    is_profile: bool,
    profile_name: Option<&str>,
    version: &str,
) -> GameDirectories {
    // Game directory for saves/config (profile-specific or vanilla)
    let mc_dir = if is_profile && profile_name.is_some() {
        // Profile-based launch: use isolated directory for saves/config
        base_dir.join("instances").join(profile_name.unwrap())
    } else {
        // Vanilla launch: use default directory for saves/config
        base_dir.clone()
    };

    let assets_dir = base_dir.join("assets");
    let libraries_dir = base_dir.join("libraries");
    let natives_dir = base_dir.join("versions").join(version).join("natives");

    GameDirectories {
        base_dir,
        mc_dir,
        assets_dir,
        libraries_dir,
        natives_dir,
    }
}

/// Emits a log message to the frontend
pub fn emit_log(app: &tauri::AppHandle, message: impl Into<String>) {
    let _ = app.emit("launch-log", json!({
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
        "message": message.into()
    }));
}
