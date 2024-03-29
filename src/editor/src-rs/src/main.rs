#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod project;

use tauri::api::process::Command;

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .setup(|_| {
            Command::new_sidecar("rustdoc_web")?.spawn()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::genrs,
            command::open_project,
            command::save_project,
            command::export_project,
            command::create_subprocess,
            command::connect_subprocess,
            command::finish_subprocess,
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
