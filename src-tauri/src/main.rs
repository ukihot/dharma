use crate::controller::{fetch_results, regster_pre_game};

mod controller;
mod infra;
mod models;
mod services;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_results, regster_pre_game])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
