use super::{team_id::TeamId, team_name::TeamName, team_player::Player, team_score::TeamScore};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub id: TeamId,
    pub name: TeamName,
    pub total_score: TeamScore,
    pub members: Vec<Player>,
}

impl Team {
    pub fn new(
        id: TeamId,
        name: TeamName,
        total_score: TeamScore,
        members: Vec<Player>,
    ) -> Result<Self> {
        Ok(Self {
            id,
            name,
            total_score,
            members,
        })
    }

    /// スコアを加算する
    pub fn add_score(&mut self, score: TeamScore) -> Result<Self> {
        self.total_score += score;
        Ok(Self)
    }
}
