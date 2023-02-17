use crate::{
    models::pre_game::pre_game_repository_trait::{HavePreGameRepository, PreGameRepository},
    services::payload::pre_game_payload::PreGamePayload,
};

pub trait PreGameUsecase: HavePreGameRepository {
    fn register(&self, pre_game_payload: PreGamePayload) {
        self.provide_repository().save(pre_game_payload);
    }
}

impl<T: HavePreGameRepository> PreGameUsecase for T {}

pub trait HavePreGameUsecase {
    type PreGameUsecase: PreGameUsecase;
    fn provide_usecase(&self) -> Self::PreGameUsecase;
}
