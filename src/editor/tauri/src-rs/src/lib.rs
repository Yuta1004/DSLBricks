#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows",
)]

mod command;
mod project;

pub fn exec() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::genrs,
            command::open_project,
            command::save_project,
            command::export_project,
            command::create_verify_process,
            command::connect_verify_process,
            command::finish_verify_process,
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
