use crate::model::error::type_error::MyError;
use super::{team_id::TeamId, team_player::Player};

/// チームエンティティ
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub players: Vec<Player>,
}

impl Team {
    // Teamのコンストラクタ
    pub fn new(id: TeamId, name: &str) -> Result<Self, MyError> {
        if name.chars().count() < 3 {
            return Err(MyError::TypeError("チーム名は3文字以上です".to_owned()));
        }
        Ok(Self {
            id,
            name: name.to_string(),
            players: vec![],
        })
    }
}
