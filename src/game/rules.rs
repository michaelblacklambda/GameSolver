pub trait GameRules {
    fn possible_moves(&self) -> Vec<Self>
    where
        Self: Sized;

    fn is_game_over(&self) -> bool;

    fn is_winning_state(&self) -> bool;

    fn reward_value(&self, player: super::game::GamePlayer) -> i64;
}
