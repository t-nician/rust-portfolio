use lib::object::XYVector;

mod tool;
mod lib;
mod cli;
mod ui;

fn main() {

    let mut obj = lib::object::Entity::new(
        XYVector::new(2, 2),
        XYVector::new(3, 3)
    );

    obj.resize(XYVector::new(4, 4));
    obj.push(XYVector::new(3, 3));

    let (pos_x, pos_y, siz_x, siz_y) = (obj.position.x, obj.position.y, obj.size.x, obj.size.y);

    println!("{pos_x} {pos_y} {siz_x} {siz_y}");

    

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
