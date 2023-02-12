use anyhow::Result;
use validator::Validate;

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Validate)]
pub struct PlayerNumber {
    #[validate(range(min = 0, max = 99))]
    value: u8,
}

impl PlayerNumber {
    pub fn new(value: u8) -> Result<Self> {
        Ok(Self { value })
    }
}
