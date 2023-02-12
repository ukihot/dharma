use crate::models::player::player_entity::Player;

#[derive(Debug, Clone)]
pub struct PlayerDTO {
    pub id: String,
    pub name: String,
}

impl PlayerDTO {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

impl From<Player<'_>> for PlayerDTO {
    fn from(player: Player) -> Self {
        Self {
            id: *player.id.into(),
            name: *player.name.into(),
        }
    }
}
