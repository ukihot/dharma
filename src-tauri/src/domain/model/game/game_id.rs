use std::convert::TryFrom;

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct GameId(i32);

/// u16 から GameId への変換。
impl TryFrom<i32> for GameId {
    type Error = ();

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        Ok(Self(n))
    }
}

/// GameId から u16 への変換処理の振る舞いを定義。
impl From<GameId> for i32 {
    fn from(n: GameId) -> Self {
        n.0
    }
}
