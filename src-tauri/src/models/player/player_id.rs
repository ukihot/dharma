use std::convert::TryFrom;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerId<'a>(&'a str);

impl PlayerId<'_> {
    pub fn new<T>(id: &str) -> Self
    where
        T: Into<String>,
    {
        Self(id)
    }
}

/// PlayerId 変換
impl TryFrom<&str> for PlayerId<'_> {
    type Error = ();

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        Ok(Self(id))
    }
}

/// スライス変換
impl From<PlayerId<'_>> for &str {
    fn from(id: PlayerId) -> Self {
        id.into()
    }
}
