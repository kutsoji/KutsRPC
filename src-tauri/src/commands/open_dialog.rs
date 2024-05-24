use std::path::Path;

use percent_encoding::{
    utf8_percent_encode,
    NON_ALPHANUMERIC,
};
use serde::{
    Deserialize,
    Serialize,
};
use tauri::api::{
    dialog::blocking::FileDialogBuilder,
    path,
};

#[derive(Serialize, Deserialize)]
pub enum DialogTypes {
    File,
    Folder,
}
#[tauri::command]
pub fn open_dialog(dialog_type: DialogTypes) -> Option<String> {
    match dialog_type {
        DialogTypes::File => {
            let d = FileDialogBuilder::new()
                .set_directory(path::download_dir().unwrap_or(Path::new("").to_path_buf()))
                .set_title("Select Picture")
                .add_filter("Images", &["jpg", "jpeg", "png", "svg", "webp"]);
            match d.pick_file() {
                Some(p) => Some(
                    utf8_percent_encode(p.as_path().to_str().unwrap(), NON_ALPHANUMERIC)
                        .to_string(),
                ),
                None => None,
            }
        }
        DialogTypes::Folder => {
            let d = FileDialogBuilder::new()
                .set_directory(path::home_dir().unwrap_or(Path::new("").to_path_buf()))
                .set_title("Select Directory");
            match d.pick_folder() {
                Some(p) => Some(p.into_os_string().into_string().unwrap()),
                None => None,
            }
        }
    }
}
