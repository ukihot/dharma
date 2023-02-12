use anyhow::Result;
use validator::Validate;

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Validate)]
pub struct PlayerWeight {
    #[validate(range(max = 85))]
    value: u8,
}

impl PlayerWeight {
    pub fn new(id: u8) -> Result<Self> {
        Ok(Self { value: id })
    }
}
