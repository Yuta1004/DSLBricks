use std::fs::File;
use std::io::Write;

use tauri::InvokeError;
use rfd::FileDialog;

use crate::project::Project;

#[tauri::command]
pub fn save_project(xml: &str) -> Result<(), InvokeError> {
    let path = FileDialog::new()
        .add_filter("DSLBricks Project (.dbp)", &["dbp"])
        .set_file_name("mydsl.dbp")
        .save_file()
        .ok_or(InvokeError::from("File not specified."))?;

    let project = Project::new(xml.to_string());
    let project = serde_xml_rs::to_string(&project).unwrap();

    let mut f = File::create(path).unwrap();
    write!(&mut f, "{}", project).unwrap();

    Ok(())
}
