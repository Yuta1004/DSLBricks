use tauri::InvokeError;

#[tauri::command]
pub fn open_project() -> Result<String, InvokeError> {
    println!("open");
    Ok("open_project".to_string())
}
