use super::{player_entity::Player, player_repository::PlayerSearchable};
use anyhow::Result;
pub struct PlayerService<T>
where
    T: PlayerSearchable,
{
    player_repository: T,
}

impl<T: PlayerSearchable> PlayerService<T> {
    pub fn exists(&self, player: &Player) -> Result<bool> {
        match self.player_repository.find_by_id(&player.id) {
            _ => Ok(true),
        }
    }
}
