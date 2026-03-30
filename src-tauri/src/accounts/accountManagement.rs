use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};
use tauri::Manager;

/// Gets the path to the accounts.json file
/// 
/// On Windows, uses APPDATA/.flint/accounts.json
/// On other platforms, uses app data directory
fn accounts_file_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            return Ok(PathBuf::from(appdata).join(".flint").join("accounts.json"));
        }
    }

    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("accounts.json"))
}

/// Loads account data from file, handling format migration
fn load_accounts_data(path: &PathBuf) -> Result<Value, String> {
    if !path.exists() {
        return Ok(json!({"accounts": [], "current": null}));
    }

    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let parsed: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;

    // Handle migration from old array format to new object format
    let data = if parsed.is_array() {
        json!({"accounts": parsed, "current": null})
    } else {
        parsed
    };

    Ok(data)
}

/// Saves account data to file
fn save_accounts_data(path: &PathBuf, data: &Value) -> Result<(), String> {
    fs::write(path, serde_json::to_string(data).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

/// Creates a new account with the given username
/// 
/// Maximum 6 accounts allowed. Returns error if username already exists.
#[tauri::command]
pub fn accountcreate(app: tauri::AppHandle, username: String) -> Result<String, String> {
    let trimmed = username.trim().to_string();
    if trimmed.is_empty() {
        return Err("Empty username".into());
    }

    let path = accounts_file_path(&app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut data = load_accounts_data(&path)?;
    let accounts = data["accounts"].as_array_mut().ok_or("Invalid data structure")?;

    if accounts.len() >= 6 {
        return Err("Maximum 6 accounts allowed".into());
    }

    if accounts.iter().any(|acc| acc.as_str() == Some(&trimmed)) {
        return Err("Username already exists".into());
    }

    accounts.push(Value::String(trimmed.clone()));
    save_accounts_data(&path, &data)?;

    Ok(trimmed)
}

/// Retrieves all existing accounts
#[tauri::command]
pub fn accountget(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let path = accounts_file_path(&app)?;
    let data = load_accounts_data(&path)?;

    if let Some(accounts) = data["accounts"].as_array() {
        Ok(accounts
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect())
    } else {
        Ok(vec![])
    }
}

/// Gets the currently selected account
#[tauri::command]
pub fn accountgetcurrent(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let path = accounts_file_path(&app)?;
    let data = load_accounts_data(&path)?;

    Ok(data["current"].as_str().map(String::from))
}

/// Sets the currently selected account
/// 
/// Fails if the account does not exist
#[tauri::command]
pub fn accountsetcurrent(app: tauri::AppHandle, username: String) -> Result<(), String> {
    let path = accounts_file_path(&app)?;
    let mut data = load_accounts_data(&path)?;

    let accounts = data["accounts"].as_array().ok_or("Invalid data structure")?;
    if !accounts.iter().any(|acc| acc.as_str() == Some(&username)) {
        return Err("Account not found".into());
    }

    data["current"] = Value::String(username);
    save_accounts_data(&path, &data)?;

    Ok(())
}

/// Deletes an account by username
/// 
/// Cannot delete the currently selected account
#[tauri::command]
pub fn accountdelete(app: tauri::AppHandle, username: String) -> Result<(), String> {
    let path = accounts_file_path(&app)?;
    let mut data = load_accounts_data(&path)?;

    let current = data["current"].as_str();
    if current == Some(&username) {
        return Err("Cannot delete currently selected account".into());
    }

    let accounts = data["accounts"]
        .as_array_mut()
        .ok_or("Invalid data structure")?;
    accounts.retain(|acc| acc.as_str() != Some(&username));

    save_accounts_data(&path, &data)?;

    Ok(())
}
