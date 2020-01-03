use tic_tac_rust::{Difficulty, State};
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
