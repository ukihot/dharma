use crate::models::result::result_repository_trait::ResultRepository;
use crate::services::payload::result_payload::ResultPayload;
pub trait FetchResultsUsecase {
    fn fetch_results() -> Result<Vec<ResultPayload>, String>;
}

pub struct FetchResultsInteractor<U: FetchResultsUsecase, R: ResultRepository> {
    query_service: U,
    repository: R,
}

impl<U: FetchResultsUsecase, R: ResultRepository> FetchResultsInteractor<U, R> {
    pub fn new(query_service: U, repository: R) -> Self {
        FetchResultsInteractor {
            query_service,
            repository,
        }
    }
}
