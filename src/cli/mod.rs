mod conways_game_of_life;
mod number_guessing_game;
mod protected_number;
mod jump_game;

use core::time;
use std::{io, thread};
use rand::Rng;

use number_guessing_game::NumberGuessingGaming;
use conways_game_of_life::ConwaysGame;


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


pub fn protected_number_loop() {
    let mut protected_number = protected_number::ProtectedNumber::new(rand::thread_rng().gen::<i64>());

    loop {
        let current_number = protected_number.get_number();
        println!("Number: {current_number}\nTo refresh page, press enter OR enter a number to change the protected numbers value.\nEnter 'exit' to leave.");

        let input = get_input();

        if input.trim().to_lowercase() == "exit" {
            break
        }

        match input.trim().parse::<i64>() {
            Ok(number) => { protected_number.set_number(number) },
            Err(_) => {}
        }
    }
}


pub fn conways_game_loop() {
    let mut game = ConwaysGame::new();

    loop {
        game.game_step();
        game.display_grid();

        println!("Enter 'exit' to break the loop, hold enter and watch!");
        let result = get_input();

        if result.trim().to_lowercase() == "exit" {
            break;
        }

    }
    //game.game_step();
    //game.display_grid();
}


pub fn jump_game_loop() {
    let mut game = jump_game::JumpGame::new();
    let delay = time::Duration::from_millis(16);

    loop {
        game.update_and_display();
        
        thread::sleep(delay);
    }
}