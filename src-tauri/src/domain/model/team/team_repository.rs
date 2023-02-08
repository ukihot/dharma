use super::team_entity::Team;

pub trait TeamRepository {
    fn list(&self) -> Vec<Team>;
}