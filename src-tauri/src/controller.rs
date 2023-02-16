use crate::services::{
    payload::{pre_game_payload::PreGamePayload, result_payload::ResultPayload},
    usecases::register_pre_game_usecase::RegisterPreGameUsecase,
};

#[tauri::command]
pub fn fetch_results() -> Result<Vec<ResultPayload>, String> {
    let usecase = ResultUsecase::new();
    let res = usecase.fetch_game();
    res.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn regster_pre_game(pre_game_payload: PreGamePayload) -> String {
    // usecaseにpayload丸投げ
    let usecase = RegisterPreGameUsecase::new(pre_game_payload);
    format!("Success")
}
