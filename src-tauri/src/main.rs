use crate::services::fetch_games;

mod common;
mod infra;
mod models;
mod services;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_games])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
