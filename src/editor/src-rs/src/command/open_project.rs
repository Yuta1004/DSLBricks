use std::fs;

use tauri::InvokeError;
use rfd::FileDialog;

use crate::project::Project;

#[tauri::command]
pub fn open_project() -> Result<Option<String>, InvokeError> {
    let path = FileDialog::new()
        .add_filter("DSLBricks Project (.dbp)", &["dbp"])
        .set_file_name("mydsl.dbp")
        .pick_file()
        .ok_or(InvokeError::from("File not selected."))?;

    let project_file = fs::read_to_string(path).unwrap();
    let project = serde_xml_rs::from_str::<Project>(&project_file).unwrap();

    Ok(Some(project.blockly_xml.to_string()))
}
