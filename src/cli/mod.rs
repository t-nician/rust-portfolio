mod number_guessing_game;

use std::io;
use number_guessing_game::NumberGuessingGaming;


pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Input expected!");

    return input;
}


pub fn number_guessing_game_loop() {
    let mut guessing_game = NumberGuessingGaming::new();

    guessing_game.new_number();

    loop {
        guessing_game.prompt_guess();

        let input = get_input();

        if input.trim().to_lowercase() == "exit" {
            break
        }

        match input.trim().parse::<i64>() {
            Ok(number) => { guessing_game.guess_number(number); },
            Err(_) => { }
        }
    }
}