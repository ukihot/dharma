use std::convert::TryFrom;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(i32);

impl PlayerId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

impl TryFrom<i32> for PlayerId {
    type Error = ();

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        Ok(Self(n))
    }
}

impl From<PlayerId> for i32 {
    fn from(n: PlayerId) -> Self {
        n.0
    }
}
