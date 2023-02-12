use crate::models::player::player_entity::Player;

use super::{team_entity::Team, team_score::TeamScore};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TeamRepository: Send + Sync + 'static {
    /// 点数加算
    async fn add(&self, team: Team, point: TeamScore) -> Result<TeamScore>;

    /// メンバー追加
    async fn register(&self, team: Team, member: Player) -> Result<()>;
}
