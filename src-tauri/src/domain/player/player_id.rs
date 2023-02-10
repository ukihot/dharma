use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamPlayerId(pub i32);

impl TeamPlayerId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }
}

/// u16 から TeamPlayerId への変換。
impl TryFrom<i32> for TeamPlayerId {
    type Error = ();

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        Ok(Self(n))
    }
}

/// TeamPlayerId から i32 への変換処理の振る舞いを定義。
impl From<TeamPlayerId> for i32 {
    fn from(n: TeamPlayerId) -> Self {
        n.0
    }
}
