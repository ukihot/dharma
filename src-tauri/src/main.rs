use crate::services::add_score;

mod common;
mod infra;
mod models;
mod services;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_score])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
