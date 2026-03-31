use std::process::{Command, Stdio};
use std::path::PathBuf;
use tauri::Manager;

/// Configuration for launching Minecraft
pub struct LaunchConfig {
    pub java_exe: String,
    pub main_class: String,
    pub classpath: String,
    pub java_library_path: String,
    pub version: String,
    pub username: String,
    pub asset_index: String,
    pub game_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub ram_mb: u32,
    pub custom_jvm_args: String,
    pub game_width: u32,
    pub game_height: u32,
    pub fullscreen: bool,
    pub keep_launcher_background: bool,
}

/// Spawns the Minecraft Java process with the given configuration
pub async fn spawn_minecraft_process(
    app: &tauri::AppHandle,
    config: LaunchConfig,
) -> Result<(), String> {
    super::pathManagement::emit_log(app, "Launching game...");

    let mut cmd = Command::new(&config.java_exe);

    let max_ram = format!("-Xmx{}M", config.ram_mb);
    let min_ram = format!("-Xms{}M", config.ram_mb / 2);

    cmd.arg(&max_ram)
        .arg(&min_ram);
    
    // Add custom JVM arguments if provided
    if !config.custom_jvm_args.is_empty() {
        for arg in config.custom_jvm_args.split_whitespace() {
            cmd.arg(arg);
        }
    }

    cmd.arg(&config.java_library_path)
        .arg("-cp")
        .arg(&config.classpath)
        .arg(&config.main_class)
        .arg("--version")
        .arg(&config.version)
        .arg("--gameDir")
        .arg(
            config
                .game_dir
                .to_str()
                .ok_or("Invalid game directory path")?,
        )
        .arg("--assetsDir")
        .arg(
            config
                .assets_dir
                .to_str()
                .ok_or("Invalid assets directory path")?,
        )
        .arg("--assetIndex")
        .arg(&config.asset_index)
        .arg("--uuid")
        .arg("00000000-0000-0000-0000-000000000000")
        .arg("--accessToken")
        .arg("0")
        .arg("--enable-native-access=ALL-UNNAMED")
        .arg("--username")
        .arg(&config.username)
        .arg("--userType")
        .arg("legacy")
        .arg("--versionType")
        .arg("release");
    
    // Add game resolution if set
    if config.game_width > 0 && config.game_height > 0 {
        cmd.arg("--width").arg(config.game_width.to_string())
           .arg("--height").arg(config.game_height.to_string());
    }
    
    // Add fullscreen flag if enabled
    if config.fullscreen {
        cmd.arg("--fullscreen");
    }

    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = cmd.spawn().map_err(|e| {
        let msg = format!("Failed to launch game: {}", e);
        super::pathManagement::emit_log(app, format!("[ERROR] {}", msg));
        msg
    })?;

    let pid = child.id();
    super::pathManagement::emit_log(
        app,
        format!(
            "Game launched with PID: {}",
            pid
        ),
    );

    // Spawn a background task to wait for the game process and report completion
    let app_clone = app.clone();
    let keep_launcher_background = config.keep_launcher_background;
    tokio::spawn(async move {
        match child.wait() {
            Ok(status) => {
                if status.success() {
                    super::pathManagement::emit_log(&app_clone, format!("[OK] Game exited successfully (PID: {})", pid));
                } else {
                    let exit_code = status.code().map(|c| c.to_string()).unwrap_or_else(|| "unknown".to_string());
                    super::pathManagement::emit_log(&app_clone, format!("[ERROR] Game crashed with exit code: {} (PID: {})", exit_code, pid));
                }
                
                // If keep_launcher_background is true, show the launcher window
                if keep_launcher_background {
                    // Get all webview windows and show them
                    for window in app_clone.webview_windows().values() {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            },
            Err(e) => {
                super::pathManagement::emit_log(&app_clone, format!("[ERROR] Failed to wait for game process: {} (PID: {})", e, pid));
            }
        }
    });

    Ok(())
}
