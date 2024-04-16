mod term;


pub fn draw_loop() {
    let mut engine = term::DisplayEngine::new(
        10, 
        10, 
        1
    );

    engine.create_platform(
        10, 
        1, 
        1, 
        1
    );

    engine.start_display_loop();
}