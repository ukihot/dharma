use crate::model::player::player_entity::Player;

/// チームエンティティ
pub struct Team {
    pub id: i32,
    pub name: String,
    pub members: Vec<Player>,
}

impl Team {
    // Teamのコンストラクタ
    pub fn new(name: &str) -> Result {
        if name.chars().count() < 3 {
            return Err(MyError::type_error("チーム名は3文字以上です"));
        }
        Ok(Self {
            name: name.to_string(),
        })
    }
}