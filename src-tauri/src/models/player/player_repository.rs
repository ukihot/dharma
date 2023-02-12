use anyhow::Result;
use async_trait::async_trait;
use super::{player_entity::Player, player_id::PlayerId};

#[async_trait]
pub trait PlayerSavable: Clone + Send + Sync + 'static {
    async fn save(&self, player: Player) -> Result<()>;
}

#[async_trait]
pub trait PlayerSearchable: Clone + Send + Sync + 'static {
    async fn find_by_id(&self, id: &PlayerId) -> Result<Player>;
}
