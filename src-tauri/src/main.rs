use crate::services::greet;
use crate::services::line_out;
use crate::services::raid_out;
use crate::services::raid_success;

mod common;
mod infra;
mod models;
mod services;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let _scorer_window =
                tauri::WindowBuilder::new(app, "local", tauri::WindowUrl::App("index.html".into()))
                    .build()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            raid_out,
            raid_success,
            line_out
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
