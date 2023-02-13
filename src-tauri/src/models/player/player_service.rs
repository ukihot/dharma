use super::{player_entity::Player, player_repository::PlayerRepository};
use anyhow::Result;
pub struct PlayerService<T>
where
    T: PlayerRepository,
{
    player_repository: T,
}

impl<T: PlayerRepository> PlayerService<T> {
    pub fn exists(&self, player: &Player) -> Result<bool> {
        match self.player_repository.find_by_id(&player.id) {
            _ => Ok(true),
        }
    }
}
