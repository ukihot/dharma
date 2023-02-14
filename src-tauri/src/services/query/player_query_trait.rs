use crate::services::dto::player_dto::PlayerDTO;

// QueryServiceトレイトを定義
pub trait PlayerQueryService {
    fn fetch_by_id(&self, player_id: String) -> Vec<PlayerDTO>;
}
