use anyhow::Result;
use async_trait::async_trait;

use crate::services::dto::player_dto::PlayerDTO;
#[async_trait]
pub trait RegisterPlayerCommand: Send + Sync + 'static {
    async fn register(&self, player_data: PlayerDTO) -> Result<PlayerDTO>;
}
