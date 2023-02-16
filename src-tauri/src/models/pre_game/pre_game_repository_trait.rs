use crate::services::payload::pre_game_payload::PreGamePayload;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PreGameRepository {
    // 試合予定の登録
    async fn save(&self, pre_game_payload: PreGamePayload) -> Result<()>;
}
