use anyhow::Result;

use crate::models::player::{player_id::PlayerId, player_status::PlayerStatus};

pub struct PlayerParams {
    pub name: String,
    pub gender: String,
    pub weight: i32,
    pub height: i32,
}

pub trait PlayerUsecase {
    fn register_player(&self, params: PlayerParams) -> Result<()>;

    fn update_player_status(&self, player_id: PlayerId, next_status: PlayerStatus) -> Result<()>;
}
