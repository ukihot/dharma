use std::convert::TryFrom;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Round(u8);

impl Round {
    pub fn new(id: u8) -> Self {
        Self(id)
    }
}

impl TryFrom<u8> for Round {
    type Error = ();

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        Ok(Self(n))
    }
}

impl From<Round> for u8 {
    fn from(n: Round) -> Self {
        n.0
    }
}
