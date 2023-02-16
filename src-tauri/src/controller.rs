use crate::services::payload::result_payload::ResultPayload;

#[tauri::command]
pub fn fetch_results() -> Result<Vec<ResultPayload>, String> {
    let usecase = ResultUsecase::new();
    let res = usecase.fetch_game();
    res.map_err(|e| e.to_string())
}
