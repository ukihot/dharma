use super::{game_time::GameTime, raid_scenario::RaidScenario};
use crate::models::player::player_entity::Player;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kabbadi {
    pub round: u8,
    pub result: RaidScenario,
    pub raider: Player,
    pub anti: Vec<Player>,
    pub remaining_time: GameTime,
}

impl Kabbadi {
    pub fn new(
        round: u8,
        result: RaidScenario,
        raider:  Player,
        anti: Vec<Player>,
        remaining_time: GameTime,
    ) -> Result<Self> {
        Ok(Self {
            round,
            result,
            raider,
            anti,
            remaining_time,
        })
    }
}
