#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerRole {
    value: Role,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Role {
    Raider,
    Anti,
    Allrounder,
}