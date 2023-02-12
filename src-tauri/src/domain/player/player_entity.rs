use anyhow::Result;

use super::{
    player_height::PlayerHeight, player_id::PlayerId, player_name::PlayerName,
    player_number::PlayerNumber, player_role::PlayerRole, player_status::PlayerStatus,
    player_weight::PlayerWeight,
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub id: PlayerId,
    pub name: PlayerName,
    pub role: PlayerRole,
    pub height: PlayerHeight,
    pub weight: PlayerWeight,
    pub number: PlayerNumber,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(
        id: PlayerId,
        name: PlayerName,
        role: PlayerRole,
        height: PlayerHeight,
        weight: PlayerWeight,
        number: PlayerNumber,
        status: PlayerStatus,
    ) -> Result<Self> {
        Ok(Self {
            id,
            name,
            height,
            weight,
            number,
            status,
            role,
        })
    }
}
