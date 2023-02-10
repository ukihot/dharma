use std::ops::AddAssign;
use validator::Validate;
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Validate)]
pub struct TeamScore {
    #[validate(range(min = 1))]
    value: u8,
}

impl AddAssign for TeamScore {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}
