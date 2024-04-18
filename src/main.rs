use lib::object::*;

mod tool;
mod lib;
mod cli;
mod ui;

fn main() {
    let mut pool = EntityPool::new();
    
    let entity_uuid_a = pool.create_entity(XYVector::new(2, 2), XYVector::new(1, 1));
    let entity_uuid_b = pool.create_entity(XYVector::new(1, 1), XYVector::new(1, 1));

    for entity in pool.get_colliding_entities(entity_uuid_a) {
        let uuid = entity.uuid;

        println!("{uuid}");
    }



    let options: Vec<&str> = vec!["NumberGuessingGame", "ProtectedNumber", "ConwaysGameOfLife"];
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

            _ => { println!("Invalid option!") }
        }
    }

}
