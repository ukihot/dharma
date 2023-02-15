pub mod dto;
pub mod interactor;
pub mod query;
pub mod usecase;
use tauri::{Event, Manager, Window};
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct AddScorePayload {
    pub current_score: u8,
}

#[tauri::command]
pub fn add_score(payload: AddScorePayload) -> u8 {
    payload.current_score + 1
}

fn format_invocation(window: &Window, event: &str, payload: String) -> Result<String, String> {
    let js_string = format!(r#"{}"#, payload);
    let invocation = format!(r#"{}("#, event);

    let js_code = format!(
        r#"try {{ window['{}']({}) }} catch(e) {{ console.error(e) }}"#,
        event, js_string
    );
    Ok(js_code)
}

/// タッチ成功：
/// プレーヤーが所属するチームへの加点（タッチした人数分＋ボーナスの有無）
/// タッチされたアンティのステータスをアウトに更新
/// レイダーチームの復活するプレーヤーのステータスをインに更新
#[tauri::command]
pub fn raid_success() -> String {
    format!("")
}

/// レイドアウト：
/// 相手チームへの加点（スーパータックルの有無）
/// レイダーのステータスをアウトに更新
/// アンティチームの復活するプレーヤーのステータスをインに更新
#[tauri::command]
pub fn raid_out() -> String {
    format!("")
}

/// ラインアウト：
/// 相手チームへの加点
/// 特定プレイヤーのステータスをアウトに更新
#[tauri::command]
pub fn line_out() -> String {
    format!("")
}
