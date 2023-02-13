pub struct Command;

pub trait KabaddiUsecase {
    fn calculate_score(&self) -> u8;
}

impl KabaddiUsecase for Command {
    /// シナリオから点数を決定
    fn calculate_score(&self) -> u8 {
        1
    }
}
