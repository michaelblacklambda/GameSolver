use super::{rules::GameRules, state::GameState};

pub trait GameStrategy {
    fn make_move<S>(game: S) -> S
    where
        S: GameRules + GameState + Clone + Send + Sync;
}
