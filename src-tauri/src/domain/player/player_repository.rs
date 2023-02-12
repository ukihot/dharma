use anyhow::Result;
use async_trait::async_trait;

use super::player_entity::Player;

#[async_trait]
pub trait PlayerSavable: Send + Sync + 'static {
    async fn save(&self, player: Player) -> Result<()>;
}
