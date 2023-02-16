use serde::{self, Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ResultPayload {
    pub id: String,
    pub winner: String,
    pub loser: String,
    pub score: String,
    pub held_at: String,
}
