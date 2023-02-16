pub mod dto;
pub mod interactor;
pub mod query;
pub mod usecase;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GameResultPayload {
    pub id: String,
    pub winner: String,
    pub loser: String,
    pub score: String,
    pub held_at: String,
}

#[tauri::command]
pub fn fetch_games() -> Result<Vec<GameResultPayload>, String> {
    Ok(vec![
        GameResultPayload {
            id: ("1").to_string(),
            winner: ("安藝ライノハーツ").to_string(),
            loser: ("尾道オータムリーブス").to_string(),
            score: ("32-31").to_string(),
            held_at: ("2023/01/02").to_string(),
        },
        GameResultPayload {
            id: ("2").to_string(),
            winner: ("尾道オータムリーブス").to_string(),
            loser: ("安藝ライノハーツ").to_string(),
            score: ("9-29").to_string(),
            held_at: ("2022/12/22").to_string(),
        },
        GameResultPayload {
            id: ("2").to_string(),
            winner: ("B.W.H").to_string(),
            loser: ("安藝ライノハーツ").to_string(),
            score: ("34-11").to_string(),
            held_at: ("2023/02/16").to_string(),
        },
    ])
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
