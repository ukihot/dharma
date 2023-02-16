use crate::{
    models::pre_game::pre_game_repository_trait::PreGameRepository,
    services::payload::result_payload::ResultPayload,
};

pub trait QueryService {
    fn fetch_results() -> Result<Vec<ResultPayload>, String>;
}

pub struct GameInteractor<Q: QueryService, R: PreGameRepository> {
    query_service: Q,
    repository: R,
}

impl<Q: QueryService, R: PreGameRepository> GameInteractor<Q, R> {
    pub fn new(query_service: Q, repository: R) -> Self {
        GameInteractor {
            query_service,
            repository,
        }
    }
}
