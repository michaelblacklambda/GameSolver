use std::sync::Arc;

use crate::game::{
    game::{self, GamePlayer},
    rules::GameRules,
    state::GameState,
    strategy::GameStrategy,
};
use rand::{prelude::*, rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub struct MCTS {}

impl MCTS {
    /* 1 if Player1 wins, -1 if Player2 wins, 0 if tie*/
    fn play_game_entirely<S>(game: S) -> i64
    where
        S: GameRules + GameState + Clone,
    {
        if game.is_winning_state() {
            match game.player_turn() {
                GamePlayer::Player1 => {
                    return -1;
                }
                GamePlayer::Player2 => {
                    return 1;
                }
            }
        }

        let possible_games = game.possible_moves();
        if possible_games.is_empty() {
            return 0;
        }

        let random_game = possible_games.choose(&mut rng()).unwrap().clone();
        MCTS::play_game_entirely(random_game)
    }

    fn calculate_best_move<S>(game: S, player: GamePlayer) -> S
    where
        S: GameRules + GameState + Clone,
    {
        let possible_games = game.possible_moves();

        let (best_game, _) = possible_games
            .into_iter()
            .map(|possible_game| {
                /* Play possible game 1000 times and calculate the score */
                let res = (0..1000)
                    .map(|_| MCTS::play_game_entirely(possible_game.clone()))
                    .reduce(|acc, e| acc + e)
                    .unwrap();

                (possible_game, res)
            })
            .map(|(game, score)| match player {
                GamePlayer::Player1 => (game, score),
                GamePlayer::Player2 => (game, score * -1),
            })
            .reduce(|(game1, score1), (game2, score2)| {
                if score1 > score2 {
                    (game1, score1)
                } else {
                    (game2, score2)
                }
            })
            .unwrap();

        best_game
    }
}

impl GameStrategy for MCTS {
    fn make_move<S>(game: S) -> S
    where
        S: GameRules + GameState + Clone + Send + Sync,
    {
        let player = game.player_turn();
        MCTS::calculate_best_move(game, player)
    }
}
