pub mod dto;
pub mod interactor;
pub mod query;
pub mod usecase;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
