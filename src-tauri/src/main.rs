#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
mod commands;
use commands::*;
mod structs;
use std::{
    fs::create_dir_all,
    path::Path,
};
use structs::Config;
use tauri::Manager;
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let config_dir = app.path_resolver().app_config_dir().unwrap();
            let config_file = format!("{}\\config.toml", config_dir.to_str().unwrap());
            let presets_dir = format!("{}\\presets", config_dir.to_str().unwrap());
            if !Path::new(config_dir.as_os_str()).exists() {
                create_dir_all(config_dir)?;
                create_dir_all(presets_dir.clone())?;
                update_config(
                    Config {
                        app_id: None,
                        remember: false,
                        presets_dir: Some(presets_dir.clone()),
                        minimize: false,
                    },
                    app.app_handle(),
                )?;
            };
            if !Path::new(&presets_dir).exists() {
                create_dir_all(presets_dir.clone())?;
            }
            if !Path::new(&config_file).exists() {
                update_config(
                    Config {
                        app_id: None,
                        remember: false,
                        presets_dir: Some(presets_dir.clone()),
                        minimize: false,
                    },
                    app.app_handle(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_process_start_time,
            create_preset,
            update_config,
            open_dialog,
            load_config,
            load_presets,
            add_preset,
            remove_preset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
