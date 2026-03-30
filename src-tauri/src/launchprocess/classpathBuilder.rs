use std::path::PathBuf;
use serde_json::Value;

/// Builds the Java classpath from version JSON metadata
/// 
/// Collects all library JARs and the main game JAR, joining them with semicolons (Windows)
pub fn build_classpath(
    version_json: &Value,
    libraries_dir: &PathBuf,
    main_jar: &PathBuf,
) -> Result<String, String> {
    let mut jars = Vec::new();

    // Process all libraries from the version JSON
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

    // Add main game JAR
    jars.push(main_jar.clone());

    // Build classpath string (semicolon-separated for Windows)
    let classpath = jars
        .iter()
        .filter_map(|p| p.to_str())
        .collect::<Vec<_>>()
        .join(";");

    Ok(classpath)
}

/// Extracts the asset index from version JSON
pub fn get_asset_index(version_json: &Value) -> Result<String, String> {
    version_json["assetIndex"]["id"]
        .as_str()
        .ok_or_else(|| "Missing assetIndex.id".to_string())
        .map(|s| s.to_string())
}

/// Extracts the main class from version JSON
pub fn get_main_class(version_json: &Value) -> Result<String, String> {
    version_json["mainClass"]
        .as_str()
        .ok_or_else(|| "Missing mainClass".to_string())
        .map(|s| s.to_string())
}
