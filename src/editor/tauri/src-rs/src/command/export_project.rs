use tauri::InvokeError;

#[tauri::command]
pub fn export_project() -> Result<String, InvokeError> {
    println!("export");
    Ok("export_proj".to_string())
}
