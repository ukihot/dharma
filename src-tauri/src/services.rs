use self::usecase::{Command, KabaddiUsecase};

pub mod dto;
pub mod usecase;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn calculate_score() -> u8 {
    let cmd = Command {};
    cmd.calculate_score()
}
