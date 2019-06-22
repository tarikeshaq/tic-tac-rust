
use gamelib::game_state::State;
use std::io;
fn main() -> Result<(), &'static str> {
    let mut state = State::new();
    let mut x_o_choice = String::new();
    println!("Please choose x or o");
    io::stdin()
        .read_line(&mut x_o_choice)
        .expect("Invalid choice");
    if x_o_choice.trim() == "x" {
        loop {
            if state.get_empty_spots().len() == 0 {
                println!("It is a tie!");
            }
            println!("Choose an index!");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Invalid input!");
            let index: usize = user_input.trim().parse().unwrap();
            state.update_board(index, 'x')?;
            if state.is_win('x') {
                println!("You won!!");
                break;
            }
            if state.get_empty_spots().len() == 0 {
                println!("It is a tie!");
            }
            let m = state.best_next_move(false);
            println!("Computer chose index: {}", m.index);
            state.update_board(m.index, 'o')?;
            if state.is_win('o') {
                println!("Computer won!!");
                break;
            }
        }
        Ok(())
    } else {
        loop {
            if state.get_empty_spots().len() == 0 {
                println!("It is a tie!");
            }
            let m = state.best_next_move(true);
            println!("Computer chose index: {}", m.index);
            state.update_board(m.index, 'x')?;
            if state.is_win('x') {
                println!("The computer wins!");
                break;
            }
            if state.get_empty_spots().len() == 0 {
                println!("It is a tie!");
            }
            println!("Choose an index!");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Invalid input!");
            let index: usize = user_input.trim().parse().unwrap();
            state.update_board(index, 'o')?;
        }
        Ok(())
    }
}
