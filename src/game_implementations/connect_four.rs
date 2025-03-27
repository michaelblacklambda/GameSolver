use std::collections::HashMap;

use itertools::Itertools;

use crate::game::game::GamePlayer;
use crate::game::rules::GameRules;
use crate::game::state::GameState;

#[derive(Debug, Clone)]
pub struct ConnectFour {
    board: Vec<Vec<Piece>>,
    red_turn: bool,
    last_move: Option<(usize, usize)>,
}

impl ConnectFour {
    fn set_position(&self, row: usize, column: usize) -> Self {
        let piece = if self.red_turn {
            Piece::Red
        } else {
            Piece::Black
        };

        let mut new_board = self.board.clone();
        new_board[row][column] = piece;
        ConnectFour {
            board: new_board,
            red_turn: !self.red_turn,
            last_move: Some((row, column)),
        }
    }

    fn get_open_row<'a>(board: &Vec<Vec<Piece>>, column: usize) -> Option<usize> {
        let row_len = board.len();

        let open_row = (0..row_len)
            .map(|row| (row, board[row][column]))
            .filter(|(_row, piece)| *piece == Piece::Empty)
            .map(|(row, _)| row)
            .max();

        open_row
    }

    fn get_all_open_rows(board: &Vec<Vec<Piece>>) -> Vec<(usize, usize)> {
        let num_cols = board.first().unwrap().len();
        (0..num_cols)
            .map(|column| (ConnectFour::get_open_row(&board, column), column))
            .filter(|(row, _column)| row.is_some())
            .map(|(row, column)| (row.unwrap(), column))
            .collect_vec()
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

    pub fn make_user_move(&self, col: usize) -> Self {
        let valid_moves = ConnectFour::get_all_open_rows(&self.board);
        let player_move = valid_moves
            .into_iter()
            .filter(|(_r, c)| c == &col)
            .collect_vec()
            .first()
            .unwrap()
            .clone();
        self.set_position(player_move.0, player_move.1)
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
        if (self.is_game_over()) {
            return Vec::new();
        }

        let board = &self.board;
        let piece = if self.red_turn {
            Piece::Red
        } else {
            Piece::Black
        };

        let new_states = ConnectFour::get_all_open_rows(&board)
            .into_iter()
            .map(|(row, column)| self.set_position(row, column))
            .collect();

        return new_states;
    }

    fn is_game_over(&self) -> bool {
        ConnectFour::get_all_open_rows(&self.board).is_empty() || self.is_winning_state()
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

    fn reward_value(&self, player: GamePlayer) -> i64 {
        let last_player = if self.red_turn {
            GamePlayer::Player1
        } else {
            GamePlayer::Player2
        };

        let winner = self.is_winning_state();
        let score = if winner && last_player == player {
            1
        } else if winner && last_player != player {
            -1
        } else {
            0
        };

        score
    }
}

impl GameState for ConnectFour {
    fn initial_state() -> Self {
        ConnectFour {
            board: vec![vec![Piece::Empty; 7]; 6],
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

    fn player_turn(&self) -> GamePlayer {
        if self.red_turn {
            GamePlayer::Player1
        } else {
            GamePlayer::Player2
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_implementations::connect_four;

    use super::*;

    #[test]
    fn test_open_row() {
        let connect_four = ConnectFour::initial_state();

        let open_row = ConnectFour::get_open_row(&connect_four.board, 0).unwrap();
        let col_count = connect_four.board.first().unwrap().len();

        assert_eq!(open_row, col_count - 1);

        for col in 0..col_count {
            let open_row = ConnectFour::get_open_row(&connect_four.board, col).unwrap();
            assert_eq!(open_row, connect_four.board.len() - 1);
        }
    }

    #[test]
    fn test_all_open_rows() {
        let connect_four = ConnectFour::initial_state();

        let open_rows = ConnectFour::get_all_open_rows(&connect_four.board);
        let num_rows = connect_four.board.len();
        let num_cols = connect_four.board.first().unwrap().len();

        for (row, column) in open_rows {
            assert_eq!(row, num_rows - 1);
        }

        let connect_four = connect_four.make_user_move(0);
        let open_rows = ConnectFour::get_all_open_rows(&connect_four.board);
        let num_rows = connect_four.board.len();
        let num_cols = connect_four.board.first().unwrap().len();

        for (row, column) in open_rows {
            if column == 0 {
                assert_eq!(row, num_rows - 2);
            } else {
                assert_eq!(row, num_rows - 1);
            }
        }
    }
}
