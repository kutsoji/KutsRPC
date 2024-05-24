use std::fs::remove_file;

#[tauri::command]
pub fn remove_preset(preset_path: &str) -> Result<String, String> {
    if let Err(e) = remove_file(preset_path) {
        return Err(format!("Failed to remove preset: {}", e));
    }
    Ok("Removed preset successfully.".to_string())
}
