
use tic_tac_rust::Difficulty;
use tic_tac_rust::State;
use std::io;
fn main() -> Result<(), String> {
    let mut diff = String::new();
    println!("Please choose the difficulty!");
    println!("1 for easy, 2 for medium and 3 for hard");
    io::stdin().read_line(&mut diff).expect("Invalid input");
    let diff_num: u32 = diff.trim().parse().unwrap();
    let difficulty: Difficulty;
    match diff_num {
        1 => {
            difficulty = Difficulty::Easy;
        }
        2 => {
            difficulty = Difficulty::Medium;
        }
        3 => {
            difficulty = Difficulty::Hard;
        }
        _ => panic!("Invalid value"),
    }

    let mut state = State::new(difficulty);
    let mut x_o_choice = String::new();
    println!("Please choose x or o");
    io::stdin()
        .read_line(&mut x_o_choice)
        .expect("Invalid choice");
    if x_o_choice.trim() == "x" {
        loop {
            if state.is_tie() {
                println!("It is a tie!");
                break;
            }
            println!("Choose an index!");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Invalid input!");
            let index: usize = user_input.trim().parse().unwrap();
            state.update_board(index, 'x');
            if state.is_win('x') {
                println!("You won!!");
                break;
            }
            if state.is_tie() {
                println!("It is a tie!");
                break;
            }
            let index = state.next_move(false);
            println!("Computer chose index: {}", index);
            state.update_board(index, 'o');
            if state.is_win('o') {
                println!("Computer won!!");
                break;
            }
        }
        Ok(())
    } else {
        loop {
            if state.is_tie() {
                println!("It is a tie!");
                break;
            }
            let index = state.next_move(true);
            println!("Computer chose index: {}", index);
            state.update_board(index, 'x');
            if state.is_win('x') {
                println!("The computer wins!");
                break;
            }
            if state.is_tie() {
                println!("It is a tie!");
                break;
            }
            println!("Choose an index!");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Invalid input!");
            let index: usize = user_input.trim().parse().unwrap();
            state.update_board(index, 'o');
            if state.is_win('o') {
                println!("You Won!");
                break;
            }
        }
        Ok(())
    }
}
