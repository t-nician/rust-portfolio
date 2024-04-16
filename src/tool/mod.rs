mod termengine;


pub fn draw_loop() {
    let mut board = termengine::DrawingBoard::new(10, 10);

    board.create_platform(2, 2, 2, 3);
    board.output_display();
    /*let mut map = termengine::TileMap::new(10, 10);

    map.draw_object(
        termengine::TileType::Platform, 
        5,
        5, 
        1,
        9
    );

    map.output_map();
    /*let mut engine = term::DisplayEngine::new(
        (20, 20),
        1
    );

    engine.create_platform((10, 1), (1, 1));
    println!("{:#?}", engine.display_pixels);*/

    engine.display_output();*/

}