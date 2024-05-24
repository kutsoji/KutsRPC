use sysinfo::System;
#[tauri::command]
pub fn get_process_start_time(name: &str) -> Result<String, String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    for process in sys.processes_by_name(name) {
        return Ok(process.start_time().to_string());
    }
    Err("Couldn't find process".to_string())
}
