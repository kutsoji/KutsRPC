use crate::structs::Config;
use std::{
    fs::File,
    io::Write,
};
use toml;
#[tauri::command]
pub fn update_config(config: Config, app_handle: tauri::AppHandle) -> Result<String, String> {
    let toml_str = match config.presets_dir {
        Some(_) => match toml::to_string(&config) {
            Ok(ts) => ts,
            Err(e) => return Err(e.to_string()),
        },
        None => {
            match toml::to_string(&Config {
                presets_dir: Some(format!(
                    "{}\\presets\\",
                    app_handle
                        .path_resolver()
                        .app_config_dir()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                )),
                ..config
            }) {
                Ok(ts) => ts,
                Err(e) => return Err(e.to_string()),
            }
        }
    };

    let mut config_file = match File::create(format!(
        "{}/config.toml",
        app_handle.path_resolver().app_config_dir().unwrap().to_str().unwrap(),
    )) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    match config_file.write_all(toml_str.as_bytes()) {
        Ok(_) => Ok(format!(
            "Updated config Successfully at {}",
            app_handle.path_resolver().app_config_dir().unwrap().as_path().to_str().unwrap()
        )),
        Err(e) => return Err(e.to_string()),
    }
}
