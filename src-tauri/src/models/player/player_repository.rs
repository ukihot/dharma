use super::{player_entity::Player, player_id::PlayerId, player_name::PlayerName};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait PlayerRepository {
    async fn find_by_id(&self, id: &PlayerId) -> Result<Option<Player>>;

    async fn find_by_name(&self, name: PlayerName) -> Result<Option<Player>>;

    async fn save(&self, player: Player) -> Result<()>;

    async fn delete(&self, player: Player) -> Result<()>;
}

#[async_trait]
pub trait HavePlayerRepository {
    type PlayerRepository: PlayerRepository;

    async fn provide_player_repository(&self) -> &Self::PlayerRepository;
}
