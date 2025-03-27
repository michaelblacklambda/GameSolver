#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GamePlayer {
    Player1,
    Player2,
}

impl GamePlayer {
    pub fn next_player(self) -> GamePlayer {
        if self == GamePlayer::Player1 {
            GamePlayer::Player2
        } else {
            GamePlayer::Player1
        }
    }
}
