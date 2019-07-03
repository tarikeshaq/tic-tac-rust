
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
}

#[wasm_bindgen]
pub struct State {
    board: Vec<Vec<char>>,
    difficulty: Difficulty,
}

struct Move {
    pub index: usize,
    pub score: i32,
}

#[wasm_bindgen]
impl State {
    pub fn new(diff: Difficulty) -> State {
        State {
            board: vec![vec!['0'; 3]; 3],
            difficulty: diff,
        }
    }
    pub fn to_string(&self) -> String {
        String::from("Not implemented")
    }

    fn get_board(&self) -> Vec<Vec<char>> {
        self.board.clone()
    }

    pub fn update_board(&mut self, index: usize, val: char) {
        if index <= 8 {
            if val == 'x' || val == 'o' || val == '0' {
                let value_orig = self.get_val_by_index(index);
                if (value_orig != 'x' && value_orig != 'o') || val == '0' {
                    self.set_val_by_index(index, val);
                }
            }
        }
    }
    pub fn get_val_by_index(&self, index: usize) -> char {
        if index < 3 {
            self.board[0][index]
        } else if index < 6 {
            self.board[1][index - 3]
        } else if index < 9 {
            self.board[2][index - 6]
        } else {
            '0'
        }
    }
    fn set_val_by_index(&mut self, index: usize, val: char) -> () {
        if index < 3 {
            let mut board = self.get_board();
            let mut inner = board[0].clone();
            inner[index] = val;
            board[0] = inner;
            self.set_board(board);
        } else if index < 6 {
            let mut board = self.get_board();
            let mut inner = board[1].clone();
            inner[index - 3] = val;
            board[1] = inner;
            self.set_board(board);
        } else {
            let mut board = self.get_board();
            let mut inner = board[2].clone();
            inner[index - 6] = val;
            board[2] = inner;
            self.set_board(board);
        }
    }
    fn set_board(&mut self, new_board: Vec<Vec<char>>) -> () {
        self.board = new_board;
    }

    pub fn is_tie(&self) -> bool {
        self.get_empty_spots().len() == 0
    }

    pub fn is_win(&self, x_or_o: char) -> bool {
        (self.board[0][0] == x_or_o && self.board[0][1] == x_or_o && self.board[0][2] == x_or_o)
            || (self.board[1][0] == x_or_o
                && self.board[1][1] == x_or_o
                && self.board[1][2] == x_or_o)
            || (self.board[2][0] == x_or_o
                && self.board[2][1] == x_or_o
                && self.board[2][2] == x_or_o)
            || (self.board[0][0] == x_or_o
                && self.board[1][0] == x_or_o
                && self.board[2][0] == x_or_o)
            || (self.board[0][1] == x_or_o
                && self.board[1][1] == x_or_o
                && self.board[2][1] == x_or_o)
            || (self.board[0][2] == x_or_o
                && self.board[1][2] == x_or_o
                && self.board[2][2] == x_or_o)
            || (self.board[0][0] == x_or_o
                && self.board[1][1] == x_or_o
                && self.board[2][2] == x_or_o)
            || (self.board[0][2] == x_or_o
                && self.board[1][1] == x_or_o
                && self.board[2][0] == x_or_o)
    }

    pub fn reset_board(&mut self, diff: Difficulty) {
        self.board = vec![vec!['0'; 3]; 3];
        self.difficulty = diff;
    }

    pub fn next_move(&mut self, is_x: bool) -> usize {
        match self.difficulty {
            Difficulty::Easy => {
                if rand::random() {
                    self.random_next_move(is_x).index
                } else {
                    self.best_next_move(is_x).index
                }
            }
            Difficulty::Medium => {
                let mut rng = thread_rng();
                let value: f64 = rng.gen();
                if value > 0.8 {
                    self.random_next_move(is_x).index
                } else {
                    self.best_next_move(is_x).index
                }

            }
            Difficulty::Hard => self.best_next_move(is_x).index,
        }
    }
    fn random_next_move(&self, _is_x: bool) -> Move {
        let empties = self.get_empty_spots();
        let random_index: usize = thread_rng().gen_range(0, empties.len() - 1);
        Move {
            score: 0,
            index: empties[random_index],
        }
    }
    fn best_next_move(&mut self, is_x: bool) -> Move {
        let player_1 = 'x';
        let player_2 = 'o';
        if self.is_win(player_1) {
            return Move {
                index: 0,
                score: 10,
            };
        }
        if self.is_win(player_2) {
            return Move {
                index: 0,
                score: -10,
            };
        }

        let empty_spots = self.get_empty_spots();
        if empty_spots.len() == 0 {
            return Move { index: 0, score: 0 };
        }
        let mut moves = Vec::new();
        for index in empty_spots {
            let mut curr_player = 'x';
            if !is_x {
                curr_player = 'o';
            }
            self.update_board(index, curr_player);
            let curr_move = self.best_next_move(!is_x);
            moves.push(Move {
                score: curr_move.score,
                index: index,
            });
            self.update_board(index, '0');
        }
        if is_x {
            let mut best_score: i32 = -10000000;
            let mut best_score_index: usize = 0;
            for value in moves {
                if value.score > best_score {
                    best_score = value.score;
                    best_score_index = value.index;
                }
            }
            Move {
                index: best_score_index,
                score: best_score,
            }
        } else {
            let mut best_score: i32 = 10000000;
            let mut best_score_index: usize = 0;
            for value in moves {
                if value.score < best_score {
                    best_score = value.score;
                    best_score_index = value.index;
                }
            }

            Move {
                index: best_score_index,
                score: best_score,
            }
        }
    }
    fn get_empty_spots(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for (index_i, row) in self.get_board().iter().enumerate() {
            for (index_j, value) in row.iter().enumerate() {
                if *value == '0' {
                    if index_i == 0 {
                        result.push(index_j);
                    } else if index_i == 1 {
                        result.push(index_j + 3);
                    } else {
                        result.push(index_j + 6);
                    }
                }
            }

        }
        result
    }
}
mod tests {
    use super::*;
    #[test]
    fn create_new_state() {
        let state = State::new(Difficulty::Hard);
        let board = state.get_board();
        assert_eq!(
            vec![
                vec!['0', '0', '0'],
                vec!['0', '0', '0'],
                vec!['0', '0', '0']
            ],
            board
        );
    }

    #[test]
    fn get_val_by_valid_index() {
        let state = State::new(Difficulty::Hard);

        let val = state.get_val_by_index(4);
        assert_eq!('0', val);
    }
    #[test]
    fn get_val_by_valid_index_changed() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        state.update_board(0, 'x');
        let val = state.get_val_by_index(0);
        assert_eq!('x', val);

        Ok(())
    }

    #[test]
    fn is_win_winner() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        state.update_board(0, 'x');
        state.update_board(1, 'x');
        state.update_board(2, 'x');

        assert!(state.is_win('x'));
        Ok(())
    }

    #[test]
    fn is_win_not_winner() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        state.update_board(0, 'x');
        state.update_board(1, 'o');
        state.update_board(2, 'x');

        assert!(!state.is_win('x'));
        Ok(())
    }

    #[test]
    fn is_win_winner_diagonal() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        state.update_board(0, 'x');
        state.update_board(4, 'x');
        state.update_board(8, 'x');
        state.update_board(1, 'o');
        state.update_board(2, 'o');
        state.update_board(5, 'o');
        assert!(state.is_win('x'));
        Ok(())
    }
    #[test]
    fn best_move_empty() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        let best_move = state.best_next_move(true);
        match best_move.index {
            0 => Ok(()),
            2 => Ok(()),
            6 => Ok(()),
            8 => Ok(()),
            _ => Err(String::from("Not the best move")),
        }
    }

    #[test]
    fn best_move_one_to_win() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        state.update_board(0, 'x');
        state.update_board(1, 'x');
        assert_eq!(2, state.best_next_move(true).index);
        Ok(())
    }

    #[test]
    fn best_move_one_to_win_diagonal() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        state.update_board(0, 'x');
        state.update_board(2, 'o');
        state.update_board(4, 'x');
        state.update_board(5, 'o');
        assert_eq!(8, state.best_next_move(true).index);
        Ok(())
    }
    #[test]
    fn best_move_two_to_win() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        state.update_board(0, 'x');
        state.update_board(4, 'o');
        state.update_board(8, 'x');
        state.update_board(2, 'o');
        assert_eq!(6, state.next_move(true));
        Ok(())
    }
    #[test]
    fn best_move_two_to_win_v2() -> Result<(), String> {
        let mut state = State::new(Difficulty::Hard);
        state.update_board(0, 'x');
        state.update_board(3, 'o');
        state.update_board(2, 'x');
        state.update_board(1, 'o');
        assert_eq!(4, state.next_move(true));
        Ok(())
    }
}
