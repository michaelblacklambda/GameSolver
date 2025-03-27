pub mod game_implementations;
mod utils;
mod game {
    pub mod game;
    pub mod rules;
    pub mod state;
    pub mod strategy;
}

mod strategies {
    pub mod brute_force;
    pub mod mcts;
}

use std::io::{self, Write};

use game::{rules::GameRules, state::GameState, strategy::GameStrategy};
use game_implementations::connect_four::ConnectFour;
use strategies::{brute_force::BruteForce, mcts::MCTS};

fn main() {
    let mut connect_four = ConnectFour::initial_state();
    println!("Started game!");

    loop {
        let mut input = String::new();

        connect_four = MCTS::make_move(connect_four);
        println!("Board: \n {}", connect_four.state_to_string());

        if connect_four.is_winning_state() {
            println!("You lost!");
            return;
        }
        if connect_four.is_game_over() {
            println!("Tie!");
        }

        println!("Enter column to drop piece in: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let col = match input.trim().parse::<usize>() {
            Ok(col) => col,
            Err(_) => {
                println!("Failed to parse!");
                return;
            }
        };

        connect_four = connect_four.make_user_move(col);

        if connect_four.is_winning_state() {
            println!("You won!");
            return;
        }

        if connect_four.is_game_over() {
            println!("Tie!");
        }
    }
}
