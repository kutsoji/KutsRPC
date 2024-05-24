use glob::glob;
use percent_encoding::{
    utf8_percent_encode,
    NON_ALPHANUMERIC,
};
use std::fs;

use crate::structs::Preset;
#[tauri::command]
pub fn load_presets(presets_dir: String) -> Result<Vec<Preset>, String> {
    let mut presets: Vec<Preset> = Vec::new();
    for entry in glob(&format!("{}/*.toml", presets_dir)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => match fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<Preset>(&content) {
                    Ok(preset) => {
                        if let Some(picture) = preset.picture {
                            presets.push(Preset {
                                picture: Some(
                                    utf8_percent_encode(&picture, NON_ALPHANUMERIC).to_string(),
                                ),
                                ..preset
                            });
                        } else {
                            presets.push(preset);
                        }
                    }
                    Err(e) => return Err(e.to_string()),
                },
                Err(e) => return Err(e.to_string()),
            },
            Err(e) => return Err(e.to_string()),
        }
    }
    presets.sort_by_key(|p| p.id);
    Ok(presets)
}
