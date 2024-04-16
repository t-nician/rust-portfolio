mod term;


pub fn draw_loop() {
    let mut engine = term::DisplayEngine::new(
        (20, 20),
        1
    );

    engine.create_platform((10, 1), (1, 1));

    
}