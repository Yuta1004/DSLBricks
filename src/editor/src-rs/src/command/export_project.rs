use std::fs::{create_dir, File};
use std::io::Write;

use tauri::InvokeError;
use rfd::FileDialog;

use crate::project::Project;

#[tauri::command]
pub fn export_project(xml: &str, rust: &str) -> Result<(), InvokeError> {
    let dir_path = FileDialog::new()
        .pick_folder()
        .ok_or(InvokeError::from("Directory not selected."))?;
    let dir_path = dir_path.join("mydsl");

    let project = Project::new(xml.to_string());
    let project = serde_xml_rs::to_string(&project).unwrap();

    let _ = create_dir(&dir_path);

    let mut f = File::create(dir_path.join("mydsl.dbp")).unwrap();
    write!(&mut f, "{}", project).unwrap();

    let mut f = File::create(dir_path.join("build.rs")).unwrap();
    write!(&mut f, "{}", rust).unwrap();

    let mut f = File::create(dir_path.join("Cargo.toml")).unwrap();
    writeln!(&mut f, "[packge]").unwrap();
    writeln!(&mut f, "name = \"mydsl\"").unwrap();
    writeln!(&mut f, "version = \"0.1.0\"").unwrap();
    writeln!(&mut f, "edition = \"2021\"\n").unwrap();
    writeln!(&mut f, "[dependencies]").unwrap();
    writeln!(&mut f, "# compiler = ...\n").unwrap();
    writeln!(&mut f, "[build-dependencies]").unwrap();
    writeln!(&mut f, "# catalog = ...\n").unwrap();

    let _ = create_dir(dir_path.join("src"));

    let mut f = File::create(dir_path.join("src/main.rs")).unwrap();
    writeln!(&mut f, "use compiler::executor::Interpreter;").unwrap();
    writeln!(&mut f, "use compiler::load_dsl;\n").unwrap();
    writeln!(&mut f, "load_dsl!();\n").unwrap();
    writeln!(&mut f, "fn main() {{").unwrap();
    writeln!(&mut f, "    let dsl = DSL::gen().unwrap();").unwrap();
    writeln!(&mut f, "    Interpreter::from(dsl).exec()").unwrap();
    writeln!(&mut f, "}}\n").unwrap();

    Ok(())
}
