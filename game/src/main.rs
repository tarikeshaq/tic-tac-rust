extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use gamelib::game_state::Difficulty;
use gamelib::game_state::State;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::process;


use std::io;
pub struct App {
    gl: GlGraphics,
    computer_score: u32,
    human_score: u32,
    computer_difficulty: Difficulty,
    game_state: State,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let size = (args.draw_size[0] / 14) as f64;
        let rec_1 = rectangle::square(0.0, 0.0, size);
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(FOREGROUND, rec_1, c.transform.trans(0.0, 0.0), gl);

            rectangle(
                FOREGROUND,
                rec_1,
                c.transform.trans(x - size / 2.0, 0.0),
                gl,
            );

            rectangle(
                FOREGROUND,
                rec_1,
                c.transform.trans(args.window_size[0] - size, 0.0),
                gl,
            );
            rectangle(
                FOREGROUND,
                rec_1,
                c.transform.trans(0.0, y - size / 2.0),
                gl,
            );
            rectangle(
                FOREGROUND,
                rec_1,
                c.transform.trans(x - size / 2.0, y - size / 2.0),
                gl,
            );
            rectangle(
                FOREGROUND,
                rec_1,
                c.transform
                    .trans(args.window_size[0] - size, y - size / 2.0),
                gl,
            );
            rectangle(
                FOREGROUND,
                rec_1,
                c.transform.trans(0.0, args.window_size[1] - size),
                gl,
            );
            rectangle(
                FOREGROUND,
                rec_1,
                c.transform
                    .trans(x - size / 2.0, args.window_size[1] - size),
                gl,
            );
            rectangle(
                FOREGROUND,
                rec_1,
                c.transform
                    .trans(args.window_size[0] - size, args.window_size[1] - size),
                gl,
            );
        });
    }
}
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Tic Tac Toe", [512, 342])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        computer_score: 0,
        human_score: 0,
        computer_difficulty: Difficulty::Easy,
        game_state: State::new(Difficulty::Easy),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            //app.update(&u);
        }

        if let Some(b) = e.press_args() {
            //app.press(&b);
        }

        if let Some(b) = e.release_args() {
            //app.release(&b);
        }
    }
    // let mut diff = String::new();
    // println!("Please choose the difficulty!");
    // println!("1 for easy, 2 for medium and 3 for hard");
    // io::stdin().read_line(&mut diff).expect("Invalid input");
    // let diff_num: u32 = diff.trim().parse().unwrap();
    // let difficulty: Difficulty;
    // match diff_num {
    //     1 => {
    //         difficulty = Difficulty::Easy;
    //     }
    //     2 => {
    //         difficulty = Difficulty::Medium;
    //     }
    //     3 => {
    //         difficulty = Difficulty::Hard;
    //     }
    //     _ => panic!("Invalid value"),
    // }

    // let mut state = State::new(difficulty);
    // let mut x_o_choice = String::new();
    // println!("Please choose x or o");
    // io::stdin()
    //     .read_line(&mut x_o_choice)
    //     .expect("Invalid choice");
    // if x_o_choice.trim() == "x" {
    //     loop {
    //         if state.get_empty_spots().len() == 0 {
    //             println!("It is a tie!");
    //             break;
    //         }
    //         println!("Choose an index!");
    //         let mut user_input = String::new();
    //         io::stdin()
    //             .read_line(&mut user_input)
    //             .expect("Invalid input!");
    //         let index: usize = user_input.trim().parse().unwrap();
    //         state.update_board(index, 'x')?;
    //         if state.is_win('x') {
    //             println!("You won!!");
    //             break;
    //         }
    //         if state.get_empty_spots().len() == 0 {
    //             println!("It is a tie!");
    //             break;
    //         }
    //         let m = state.next_move(false);
    //         println!("Computer chose index: {}", m.index);
    //         state.update_board(m.index, 'o')?;
    //         if state.is_win('o') {
    //             println!("Computer won!!");
    //             break;
    //         }
    //     }
    //     Ok(())
    // } else {
    //     loop {
    //         if state.get_empty_spots().len() == 0 {
    //             println!("It is a tie!");
    //             break;
    //         }
    //         let m = state.next_move(true);
    //         println!("Computer chose index: {}", m.index);
    //         state.update_board(m.index, 'x')?;
    //         if state.is_win('x') {
    //             println!("The computer wins!");
    //             break;
    //         }
    //         if state.get_empty_spots().len() == 0 {
    //             println!("It is a tie!");
    //             break;
    //         }
    //         println!("Choose an index!");
    //         let mut user_input = String::new();
    //         io::stdin()
    //             .read_line(&mut user_input)
    //             .expect("Invalid input!");
    //         let index: usize = user_input.trim().parse().unwrap();
    //         state.update_board(index, 'o')?;
    //     }
    //     Ok(())
    //}
}
