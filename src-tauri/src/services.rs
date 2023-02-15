pub mod dto;
pub mod interactor;
pub mod query;
pub mod usecase;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
