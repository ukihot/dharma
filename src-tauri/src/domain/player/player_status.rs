#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct PlayerStatus {
    status: Status,
}

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
enum Status {
    /// コート内でプレイ中
    Inside,
    /// シッティングブロックで待機中
    Outside,
    /// ベンチで交代待機
    Reserve,
}
