use tauri::InvokeError;

#[tauri::command]
pub fn save_project() -> Result<String, InvokeError> {
    println!("save");
    Ok("save_proj".to_string())
}
