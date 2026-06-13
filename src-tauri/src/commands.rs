// TODO: Implement commands in task 2

#[tauri::command]
pub fn read_model_config() -> Result<String, String> {
    todo!()
}

#[tauri::command]
pub fn write_model_config(_content: String) -> Result<(), String> {
    todo!()
}

#[tauri::command]
pub fn get_settings_path() -> Result<String, String> {
    todo!()
}
