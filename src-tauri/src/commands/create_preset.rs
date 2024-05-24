use crate::structs::Preset;
use percent_encoding::percent_decode_str;
use std::{
    fs::File,
    io::Write,
};
use toml;
#[tauri::command]
pub fn create_preset(preset: Preset) -> Result<String, String> {
    let toml_str = match preset.picture {
        Some(ref p) => {
            let decoded_picture_path = percent_decode_str(p).decode_utf8_lossy().into_owned();
            match toml::to_string(&Preset { picture: Some(decoded_picture_path), ..preset.clone() })
            {
                Ok(ts) => ts,
                Err(e) => return Err(e.to_string()),
            }
        }
        None => match toml::to_string(&preset) {
            Ok(ts) => ts,
            Err(e) => return Err(e.to_string()),
        },
    };
    let mut preset_file = match File::create(preset.path.clone()) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    match preset_file.write_all(toml_str.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    };
    Ok(format!("Created Preset {} Successfully at {}", preset.name, preset.path.clone()))
}
