pub trait GameState {
    fn initial_state() -> Self;

    fn state_to_string(&self) -> String;
}
