use anyhow::Result;
use validator::Validate;
#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct Player {
    pub id: PlayerId,
    pub name: PlayerName,
    pub height: PlayerHeight,
    pub weight: PlayerWeight,
    pub number: PlayerNumber,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(
        id: PlayerId,
        name: PlayerName,
        height: PlayerHeight,
        weight: PlayerWeight,
        number: PlayerNumber,
        status: PlayerStatus,
    ) -> Result<Self> {
        Ok(Self {
            name,
            id,
            height,
            weight,
            number,
            status,
        })
    }
}
