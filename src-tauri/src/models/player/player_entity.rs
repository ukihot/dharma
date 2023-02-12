use anyhow::Result;

use super::{
    player_height::PlayerHeight, player_id::PlayerId, player_name::PlayerName,
    player_number::PlayerNumber, player_role::PlayerRole, player_status::PlayerStatus,
    player_weight::PlayerWeight,
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player<'a> {
    pub id: &'a PlayerId<'a>,
    pub name: &'a PlayerName<'a>,
    pub role: PlayerRole,
    pub height: PlayerHeight,
    pub weight: PlayerWeight,
    pub number: PlayerNumber,
    pub status: PlayerStatus,
}

impl<'a> Player<'a> {
    pub fn new(
        id: &'a PlayerId,
        name: &'a PlayerName,
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
