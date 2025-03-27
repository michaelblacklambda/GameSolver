use super::game::GamePlayer;

pub trait GameState {
    fn initial_state() -> Self;

    fn state_to_string(&self) -> String;

    fn player_turn(&self) -> GamePlayer;
}
