use core::time;
use std::thread;

use self::asciiengine::Dimensions;

mod asciiengine;

pub fn draw_loop() {
    let mut engine = asciiengine::Engine::new(
        asciiengine::Dimensions::new(10, 10),
    );

    let object_uuid = engine.create_platform(
        asciiengine::Dimensions::new(10, 1), 
        asciiengine::Dimensions::new(1, 1)
    );

    let move_left = asciiengine::Dimensions::new(-1, 0);
    let move_right = asciiengine::Dimensions::new(1, 0);

    

    /*let mut display_engine = DisplayEngine::new(
        Vector2::new(10, 10)
    );

    let mut update_engine = UpdateEngine::new();

    display_engine.create_platform(
        Vector2::new(1, 1),
        Vector2::new(1, 5)
    );

    update_engine.mover(&display_engine.objects[0]);


    display_engine.display();*/

    /*let mut board = termengine::DrawingBoard::new(10, 10);
    let delay = time::Duration::from_millis(1);

    board.create_platform(2, 2, 2, 3);

    let mut offset = 0.0;
    let mut goto = 0.025;

    loop {
        if offset + goto > 8 as f64 {
            goto = -0.025;
        } else if offset + goto < 1 as f64 {
            goto = 0.025;
        }

        offset += goto;

        board.objects[0].position_y = offset as usize;
        board.output_display();

        thread::sleep(delay);
    }*/
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