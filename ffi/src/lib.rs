use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tic_tac_rust::{Difficulty, State};
lazy_static::lazy_static! {
    pub static ref STATE: Mutex<HashMap<usize, Arc<Mutex<State>>>> = Mutex::new(HashMap::new());
}

fn get_difficulty_from_num(difficulty: i32) -> Difficulty {
    match difficulty {
        0 => Difficulty::Easy,
        1 => Difficulty::Medium,
        2 => Difficulty::Hard,
        _ => Difficulty::Easy,
    }
}

#[no_mangle]
pub extern "C" fn create_new_state(difficulty: i32) -> u64 {
    let difficulty = get_difficulty_from_num(difficulty);
    STATE
        .lock()
        .unwrap()
        .insert(0, Arc::new(Mutex::new(State::new(difficulty))));
    0
}

#[no_mangle]
pub extern "C" fn is_win(handle: u64, x_or_o: char) -> u8 {
    if STATE
        .lock()
        .unwrap()
        .get(&(handle as usize))
        .unwrap()
        .lock()
        .unwrap()
        .is_win(x_or_o)
    {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn is_tie(handle: u64) -> u8 {
    if STATE
        .lock()
        .unwrap()
        .get(&(handle as usize))
        .unwrap()
        .lock()
        .unwrap()
        .is_tie()
    {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn reset_board(handle: u64, difficulty: i32) {
    let difficulty = get_difficulty_from_num(difficulty);
    STATE
        .lock()
        .unwrap()
        .get(&(handle as usize))
        .unwrap()
        .lock()
        .unwrap()
        .reset_board(difficulty);
}

#[no_mangle]
pub extern "C" fn next_move(handle: u64, is_x: bool) -> i32 {
    STATE
        .lock()
        .unwrap()
        .get(&(handle as usize))
        .unwrap()
        .lock()
        .unwrap()
        .next_move(is_x) as i32
}

#[no_mangle]
pub extern "C" fn update_board(handle: u64, index: u32, value: char) {
    STATE
        .lock()
        .unwrap()
        .get(&(handle as usize))
        .unwrap()
        .lock()
        .unwrap()
        .update_board(index as usize, value);
}

#[no_mangle]
pub extern "C" fn get_val_by_index(handle: u64, index: u32) -> u8 {
    match STATE
        .lock()
        .unwrap()
        .get(&(handle as usize))
        .unwrap()
        .lock()
        .unwrap()
        .get_val_by_index(index as usize)
    {
        '0' => 0,
        'x' => 1,
        'o' => 2,
        _ => 0,
    }
}
