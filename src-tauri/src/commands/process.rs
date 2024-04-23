use sysinfo::{
    ProcessExt,
    System,
    SystemExt,
};
#[tauri::command]
pub fn get_process_start_time(name: String) -> Result<String, String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    for process in sys.processes_by_name(name.as_str()) {
        return Ok(process.start_time().to_string());
    }
    Err("Couldn't find process".to_string())
}
