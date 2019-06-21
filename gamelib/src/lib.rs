pub mod game_state {
    pub struct State {
        board: Vec<Vec<char>>,
    }

    impl State {
        pub fn new() -> State {
            State {
                board: vec![vec!['0'; 3]; 3],
            }
        }

        pub fn to_string(&self) -> String {
            String::from("Not implemented")
        }

        pub fn get_board(&self) -> Vec<Vec<char>> {
            self.board.clone()
        }

        pub fn update_box(&mut self, index: usize, val: char) -> Result<(), &'static str> {
            if index > 8 {
                Err("Index needs to be between 0 and 8")
            } else {
                if val != 'x' && val != 'o' && val != '0' {
                    Err("invalid value, it needs to be 'x', 'o' or '0'")
                } else {
                    if let Ok(value) = self.get_val_by_index(index) {
                        if value == 'x' || value == 'o' {
                            Err("Invalid, value already selected")
                        } else {
                            self.set_val_by_index(index, val);
                            Ok(())
                        }
                    } else {
                        Err("unable to dereference!!!")
                    }
                }
            }
        }

        pub fn get_val_by_index(&self, index: usize) -> Result<char, &'static str> {
            if index < 3 {
                Ok(self.board[0][index])
            } else if index < 6 {
                Ok(self.board[1][index - 3])
            } else if index < 9 {
                Ok(self.board[2][index - 6])
            } else {
                Err("invalid index needs to be between 0 and 8")
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
                let mut inner = board[1].clone();
                inner[index - 3] = val;
                board[1] = inner;
                self.set_board(board);
            }
        }

        pub fn set_board(&mut self, new_board: Vec<Vec<char>>) -> () {
            self.board = new_board;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_new_state() {
        let state = game_state::State::new();
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
        let state = game_state::State::new();

        if let Ok(val) = state.get_val_by_index(4) {
            assert_eq!('0', val);
        } else {
            assert!(false);
        }
    }
    #[test]
    fn get_val_by_valid_index_changed() {
        let mut state = game_state::State::new();
        match state.update_box(0, 'x') {
            Ok(()) => match state.get_val_by_index(0) {
                Ok(val) => {
                    assert_eq!('x', val);
                }
                Err(message) => {
                    println!("Error: {}", message);
                    assert!(false);
                }
            },
            Err(message) => {
                println!("Error: {}", message);
                assert!(false);
            }
        }
    }
}
