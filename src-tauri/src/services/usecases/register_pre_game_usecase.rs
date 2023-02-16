use crate::{
    models::pre_game::pre_game_repository_trait::PreGameRepository,
    services::payload::pre_game_payload::PreGamePayload,
};

pub trait RegisterPreGameUsecase {
    fn register(pre_game_payload: PreGamePayload) -> String;
}

pub struct RegisterPreGameInteractor<U: RegisterPreGameUsecase, R: PreGameRepository> {
    query_service: U,
    repository: R,
}

impl<U: RegisterPreGameUsecase, R: PreGameRepository> RegisterPreGameInteractor<U, R> {
    pub fn new(query_service: U, repository: R) -> Self {
        RegisterPreGameInteractor {
            query_service,
            repository,
        }
    }
}
