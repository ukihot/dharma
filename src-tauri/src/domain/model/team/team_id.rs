use std::convert::TryFrom;

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct TeamId(i32);

/// u16 から TeamId への変換。
impl TryFrom<i32> for TeamId {
    type Error = ();

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        Ok(Self(n))
    }
}

/// TeamId から i32 への変換処理の振る舞いを定義。
impl From<TeamId> for i32 {
    fn from(n: TeamId) -> Self {
        n.0
    }
}
