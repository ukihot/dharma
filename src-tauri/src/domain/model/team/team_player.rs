use super::{team_entity::Team, team_player_id::TeamPlayerId};
use anyhow::Result;
use validator::Validate;
#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Player {
    pub name: String,
    pub id: TeamPlayerId,
}

impl Player {
    pub fn new(name: &str, team: &Team) -> Result<Self> {
        let id = Self::generate_player_id(team)?;
        Ok(Self {
            name: name.to_string(),
            id,
        })
    }

    pub fn generate_player_id(team: &Team) -> Result<TeamPlayerId> {
        let mut max_id = 0;

        for player in &team.members {
            let id = player.id.0;
            if id > max_id {
                max_id = id;
            }
        }
        Ok(TeamPlayerId(max_id + 1))
    }
}
