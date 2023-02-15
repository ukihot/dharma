use crate::{
    models::player::{
        player_id::PlayerId, player_repository_trait::PlayerRepository, player_status::PlayerStatus,
    },
    services::{
        query::player_query_trait::PlayerQueryService,
        usecase::player_usecase::{PlayerParams, PlayerUsecase},
    },
};
use anyhow::{Ok, Result};

pub struct RegisterPlayerInteractor<QS, R>
where
    QS: PlayerQueryService,
    R: PlayerRepository,
{
    player_query_service: QS,
    player_repository: R,
}

impl<QS, R> PlayerUsecase for RegisterPlayerInteractor<QS, R>
where
    QS: PlayerQueryService,
    R: PlayerRepository,
{
    fn register_player(&self, _player_params: PlayerParams) -> Result<()> {
        Ok(())
    }

    fn update_player_status(&self, _player_id: PlayerId, _next_status: PlayerStatus) -> Result<()> {
        todo!()
    }
}
