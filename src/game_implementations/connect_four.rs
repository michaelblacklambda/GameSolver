use std::collections::HashMap;

use itertools::{multizip, Itertools};

use crate::game::rules::GameRules;
use crate::game::state::GameState;

#[derive(Debug, Clone)]
pub struct ConnectFour {
    board: Vec<Vec<Piece>>,
    red_turn: bool,
    last_move: Option<(usize, usize)>,
}

impl ConnectFour {
    fn get_open_row<'a>(board: &Vec<Vec<Piece>>, column: usize) -> Option<usize> {
        let open_row = (0..7)
            .map(|indice| (indice, board[indice][column]))
            .filter(|(_indice, piece)| *piece == Piece::Empty)
            .map(|(indice, _)| indice)
            .min();
        open_row
    }

    fn get_all_open_rows<'a>(
        board: &'a Vec<Vec<Piece>>,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        (0..7)
            .map(move |column| (column, ConnectFour::get_open_row(&board, column)))
            .filter(|(_column, row)| row.is_some())
            .map(|(column, row)| (column, row.unwrap()))
    }

    fn horizontal_win(board: &Vec<Vec<Piece>>, player: Piece) -> bool {
        board
            .into_iter()
            .map(|row| {
                let mut count = 0;
                for piece in row {
                    count = if *piece == player { count + 1 } else { 0 };

                    if count == 4 {
                        return true;
                    }
                }

                return false;
            })
            .any(|x| x)
    }

    fn diagonal_up_win(board: &Vec<Vec<Piece>>, player: Piece) -> bool {
        let matrix_board = board
            .iter()
            .enumerate()
            .map(|(row_num, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(col_num, elem)| (col_num + row_num, row_num, col_num, elem))
            })
            .flatten();

        let board_hashmap = matrix_board.fold(
            HashMap::<usize, Vec<(usize, usize, Piece)>>::new(),
            |mut acc, (total, row, col, elem)| {
                acc.entry(total)
                    .or_insert_with(Vec::new)
                    .push((row, col, *elem));
                acc
            },
        );

        let new_board = board_hashmap
            .into_values()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .sorted_by(|(_, col_num_a, _), (_, col_num_b, _)| {
                        Ord::cmp(col_num_a, col_num_b)
                    })
                    .map(|(_, _, elem)| elem)
                    .collect_vec()
            })
            .collect_vec();

        ConnectFour::horizontal_win(&new_board, player)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Piece {
    Empty,
    Red,
    Black,
}

impl GameRules for ConnectFour {
    fn possible_moves(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        let board = &self.board;
        let piece = if self.red_turn {
            Piece::Red
        } else {
            Piece::Black
        };

        let new_states = ConnectFour::get_all_open_rows(&board)
            .map(|(column, row)| {
                let mut new_board = board.clone();
                new_board[row][column] = piece;
                (new_board, (row, column))
            })
            .map(|(new_board, (row, column))| ConnectFour {
                board: new_board,
                red_turn: !self.red_turn,
                last_move: Some((row, column)),
            })
            .collect();

        return new_states;
    }

    fn is_game_over(&self) -> bool {
        ConnectFour::get_all_open_rows(&self.board).next().is_none() || self.is_winning_state()
    }

    fn is_winning_state(&self) -> bool {
        let last_player = if self.red_turn {
            Piece::Black
        } else {
            Piece::Red
        };

        if ConnectFour::horizontal_win(&self.board, last_player) {
            return true;
        }

        // Transpose to transform vertical wins into horizontal wins
        let transposed_board = crate::utils::zip(self.board.clone());
        if ConnectFour::horizontal_win(&transposed_board, last_player) {
            return true;
        }

        if ConnectFour::diagonal_up_win(&self.board, last_player) {
            return true;
        }

        // Reverse all columns to transform diagonal down wins into diagional up wins
        let reversed_column_board = self
            .board
            .clone()
            .iter()
            .map(|row| row.iter().rev().map(|piece| piece.clone()).collect_vec())
            .collect_vec();
        if ConnectFour::diagonal_up_win(&reversed_column_board, last_player) {
            return true;
        }

        return false;
    }

    fn reward_value(&self) -> u64 {
        todo!()
    }
}

impl GameState for ConnectFour {
    fn initial_state() -> Self {
        ConnectFour {
            board: vec![vec![Piece::Empty; 7]; 7],
            red_turn: true,
            last_move: Option::None,
        }
    }

    fn state_to_string(&self) -> String {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|piece| match piece {
                        Piece::Red => 'R',
                        Piece::Black => 'B',
                        Piece::Empty => '.',
                    })
                    .fold(String::new(), |acc, piece| acc + &piece.to_string())
            })
            .fold(String::new(), |acc, row| acc + "\n" + &row)
    }
}
