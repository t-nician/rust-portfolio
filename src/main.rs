mod tool;
mod cli;
mod ui;


fn main() {
    let options: Vec<&str> = vec!["NumberGuessingGame", "ProtectedNumber", "ConwaysGameOfLife", "JumpGame", "Draw"];
    let mut compiled_str = String::new();

    for option in options {
        compiled_str = compiled_str + option + ", ";
    }

    loop {
        println!("Select portfolio project to launch:\n {compiled_str}");

        let input = cli::get_input();

        match input.trim().to_lowercase().replace(" ", "").as_str() {
            "numberguessinggame" => { cli::number_guessing_game_loop() }
            "protectednumber" => { cli::protected_number_loop(); }
            "conwaysgameoflife" => { cli::conways_game_loop(); }
            "jumpgame" => { cli::jump_game_loop(); }
            "draw" => { tool::draw_loop(); }

            _ => { println!("Invalid option!") }
        }
    }

}
