pub struct BruteForce {}

use rayon::prelude::*;

use crate::game::{game::GamePlayer, rules::GameRules, state::GameState, strategy::GameStrategy};

impl BruteForce {
    fn calculate_score<S>(game: S, player: GamePlayer) -> i64
    where
        S: GameRules + GameState,
    {
        let possible_games = game.possible_moves();
        if possible_games.is_empty() {
            return game.reward_value(player);
        }

        let best_game = possible_games
            .into_iter()
            .map(|possible_game| BruteForce::calculate_score(possible_game, player.next_player()))
            .map(|score| -1 * score)
            .max()
            .unwrap();

        best_game
    }

    fn calculate_best_move<S>(game: S, player: GamePlayer) -> S
    where
        S: GameRules + GameState + Clone,
    {
        let possible_games = game.possible_moves();

        let (best_game, _) = possible_games
            .into_iter()
            .map(|game| {
                let game2 = game.clone();
                (
                    game,
                    BruteForce::calculate_score(game2, player.next_player()),
                )
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

impl GameStrategy for BruteForce {
    fn make_move<S>(game: S) -> S
    where
        S: GameRules + GameState + Clone,
    {
        let player = game.player_turn();
        BruteForce::calculate_best_move(game, player)
    }
}
