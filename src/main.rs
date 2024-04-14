mod cli;
mod ui;


fn main() {

    loop {
        println!("Select portfolio project to launch:\nNumberGuessingGame, ProtectedNumber");

        let input = cli::get_input();

        match input.trim().to_lowercase().replace(" ", "").as_str() {
            "numberguessinggame" => { cli::number_guessing_game_loop() },
            "protectednumber" => { cli::protected_number_loop(); }


            _ => { println!("Invalid option!") }
        }
    }

}
