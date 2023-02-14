use super::player_entity::Player;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PlayerRepository {
    async fn save(&self, player: Player) -> Result<()>;
}
