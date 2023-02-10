use validator::Validate;

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Validate)]
pub struct PlayerName {
    #[validate(length(min = 1))]
    value: String,
}
