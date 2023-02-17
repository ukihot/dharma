use crate::{
    dependency_injection::DIContainer,
    services::payload::{pre_game_payload::PreGamePayload, result_payload::ResultPayload},
};

/// 戦績の取得コマンド
#[tauri::command]
pub fn fetch_results_init() -> Result<Vec<ResultPayload>, String> {
    let container = DIContainer::new();
    let usecase = container.provide_usecase();
    usecase.fetch_results()
}

/// 試合予定の登録コマンド
#[tauri::command]
pub fn regster_pre_game(pre_game_payload: PreGamePayload) {
    // usecaseにpayload丸投げ
    //let usecase = RegisterPreGameUsecase::new(pre_game_payload);
    //format!("Success")
}
