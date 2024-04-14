mod tool;
mod cli;
mod ui;


use cli::number_guessing_game::NumberGuessingGaming;

use std::io;

fn main() {
    
    let mut guessing_game = NumberGuessingGaming::new();
    guessing_game.new_number();

    loop {
        let mut input = String::new();
        
        guessing_game.prompt_guess();

        io::stdin().read_line(&mut input).expect("Input expected!");

        match input.trim().parse::<i64>() {
            Ok(number) => { guessing_game.guess_number(number); },
            Err(_) => { }
        }

        println!("{input}")
    }
}
