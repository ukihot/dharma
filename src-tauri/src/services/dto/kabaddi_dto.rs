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

/// Player型からDTOへの変換
impl From<Player> for PlayerDTO {
    fn from(player: Player) -> Self {
        Self {
            id: player.id.to_string(),
            name: player.name.to_string(),
        }
    }
}
