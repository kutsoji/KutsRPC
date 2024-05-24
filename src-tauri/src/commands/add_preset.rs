use std::fs::{
    copy,
    read_to_string,
};

use tauri::api::{
    dialog::blocking::FileDialogBuilder,
    path,
};

use crate::structs::Preset;

#[tauri::command]
pub fn add_preset(presets_dir: &str) -> Result<Preset, String> {
    let file_dialog = FileDialogBuilder::new()
        .set_directory(path::home_dir().unwrap_or_default())
        .set_title("Select Preset")
        .add_filter("preset", &["toml"]);

    let preset_path = match file_dialog.pick_file() {
        Some(path) => path,
        None => return Err("No File Was Selected".to_string()),
    };

    let content = match read_to_string(&preset_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read file: {}", e)),
    };

    let preset = match toml::from_str::<Preset>(&content) {
        Ok(preset) => Preset { path: format!("{}\\{}.toml", presets_dir, preset.name), ..preset },
        Err(e) => return Err(format!("Failed to parse TOML: {}", e)),
    };

    let new_preset_path = format!("{}\\{}.toml", presets_dir, preset.name);
    if let Err(e) = copy(&preset_path, &new_preset_path) {
        return Err(format!("Failed to copy preset: {}", e));
    }

    Ok(preset)
}
