mod game_implementations;
mod utils;
mod game {
    pub mod rules;
    pub mod state;
}

use game::state::GameState;
use game_implementations::connect_four::ConnectFour;

fn main() {
    let connect_four = ConnectFour::initial_state();
    println!("Initial board state: {}", connect_four.state_to_string());
}
