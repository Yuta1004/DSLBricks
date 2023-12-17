use std::fs::File;
use std::io::Write;

use tauri::InvokeError;
use rfd::FileDialog;

use crate::project::Project;

#[tauri::command]
pub fn save_project(xml: &str) -> Result<(), InvokeError> {
    let path = FileDialog::new()
        .add_filter("DSLBricks Project (.dbp)", &[".dbp"])
        .set_file_name("mydsl.dbp")
        .save_file();

    if let Some(path) = path {
        let blockly = serde_xml_rs::from_str(xml).unwrap();
        let project = Project::new(blockly);
        let project = serde_xml_rs::to_string(&project).unwrap();

        let mut f = File::create(path).unwrap();
        write!(&mut f, "{}", project).unwrap();
    }

    Ok(())
}
