use validator::Validate;

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug, Validate)]
pub struct PlayerName<'a> {
    #[validate(length(min = 1))]
    first: &'a str,
    middle: &'a str,
    #[validate(length(min = 1))]
    last: &'a str,
}

impl PlayerName<'_> {
    pub fn new<T>(f: &str, m: &str, l: &str) -> Self
    where
        T: Into<String>,
    {
        Self {
            first: f,
            middle: m,
            last: l,
        }
    }
}

/// スライス変換
impl<'a> From<PlayerName<'a>> for &str {
    fn from(name: PlayerName) -> Self {
        name.into()
    }
}
