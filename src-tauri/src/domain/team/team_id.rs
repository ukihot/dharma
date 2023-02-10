use validator::Validate;

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Validate)]
pub struct TeamId {
    #[validate(length(min = 1))]
    value: String,
}
