mod tool;
mod cli;
mod ui;

fn main() {

    loop {
        println!("Select portfolio project to launch:\nNumberGuessingGame,");

        let input = cli::get_input();

        match input.trim().to_lowercase().replace(" ", "").as_str() {
            "numberguessinggame" => { cli::number_guessing_game_loop() },

            _ => { println!("Invalid option!") }
        }
    }

}
