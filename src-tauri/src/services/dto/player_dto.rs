use crate::models::player::player_entity::Player;

#[derive(Debug, Clone)]
pub struct PlayerDTO {
    pub id: String,
    pub name: String,
    pub role: u8,
    pub height: u8,
    pub weight: u8,
    pub number: u8,
    pub status: u8,
}

/// Player型からDTOへの変換
impl From<Player> for PlayerDTO {
    fn from(player: Player) -> Self {
        Self {
            id: player.id.to_string(),
            name: player.name.to_string(),
            role: u8::from(player.role),
            height: u8::from(player.height),
            weight: u8::from(player.weight),
            number: u8::from(player.number),
            status: u8::from(player.status),
        }
    }
}
