use crate::structs::Config;
use std::fs::read_to_string;
use toml;
#[tauri::command]
pub fn load_config(app_handle: tauri::AppHandle) -> Result<Config, String> {
    let config_str = match read_to_string(format!(
        "{}/config.toml",
        app_handle.path_resolver().app_config_dir().unwrap().to_str().unwrap(),
    )) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    let config_content: Config = match toml::from_str(&config_str) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    Ok(config_content)
}
