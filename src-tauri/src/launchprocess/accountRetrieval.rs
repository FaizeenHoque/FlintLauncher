use std::fs;
use std::path::PathBuf;
use serde_json::Value;

/// Reads the current selected Minecraft account username
/// 
/// Accounts are stored in accounts.json in the base .flint directory
/// Handles migration from old array format to new object format
pub fn get_current_account(accounts_path: &PathBuf) -> Result<String, String> {
    if !accounts_path.exists() {
        return Err("No accounts found".to_string());
    }

    let raw = fs::read_to_string(accounts_path)
        .map_err(|e| format!("Failed to read accounts: {}", e))?;
    let data: Value = serde_json::from_str(&raw)
        .map_err(|e| format!("Failed to parse accounts: {}", e))?;

    // Handle migration from old array format to new object format
    let data = if data.is_array() {
        serde_json::json!({"accounts": data, "current": null})
    } else {
        data
    };

    data["current"]
        .as_str()
        .ok_or_else(|| "No account selected".to_string())
        .map(|s| s.to_string())
}

/// Retrieves the current account with optional error logging
pub fn get_current_account_with_log(
    app: &tauri::AppHandle,
    accounts_path: &PathBuf,
) -> Result<String, String> {
    get_current_account(accounts_path).map_err(|e| {
        super::pathManagement::emit_log(app, format!("[ERROR] {}", e));
        e
    })
}
