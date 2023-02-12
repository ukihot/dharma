use super::{game_time::GameTime, raid_scenario::RaidScenario};
use crate::models::player::player_entity::Player;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kabbadi<'a> {
    pub round: u8,
    pub result: RaidScenario,
    pub raider: &'a Player<'a>,
    pub anti: Vec<&'a Player<'a>>,
    pub remaining_time: GameTime,
}

impl<'a> Kabbadi<'a> {
    pub fn new(
        round: u8,
        result: RaidScenario,
        raider: &'a Player,
        anti: Vec<&'a Player>,
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
