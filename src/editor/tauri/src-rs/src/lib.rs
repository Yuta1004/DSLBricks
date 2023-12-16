#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows",
)]

mod command;

pub fn exec() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::genrs,
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
