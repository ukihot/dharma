use crate::services::greet;

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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
