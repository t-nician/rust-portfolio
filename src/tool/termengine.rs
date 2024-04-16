#[derive(Clone, Copy)]
pub enum PixelType {
    Platform,
    Player,
    Empty
}


pub struct DrawableObject {
    pixel_type: PixelType,
    position_x: usize,
    position_y: usize,
    size_x: usize,
    size_y: usize
}


impl DrawableObject {
    pub fn new(
        pixel_type: PixelType,
        position_x: usize, 
        position_y: usize, 
        size_x: usize, 
        size_y: usize
    ) -> DrawableObject {
        DrawableObject {
            pixel_type: pixel_type,
            position_x: position_x,
            position_y: position_y,
            size_x: size_x,
            size_y: size_y
        }
    }
}


pub struct DrawingBoard {
    display: Vec<Vec<PixelType>>,
    objects: Vec<DrawableObject>,
    size_x: usize,
    size_y: usize
}


impl DrawingBoard {
    pub fn new(size_x: usize, size_y: usize) -> DrawingBoard {
        DrawingBoard {
            display: Vec::new(),
            objects: Vec::new(),
            size_x: size_x,
            size_y: size_y
        }
    }

    pub fn create_platform(&mut self, position_x: usize, position_y: usize, size_x: usize, size_y: usize) {
        let new_object = DrawableObject::new(
            PixelType::Platform,
            position_x,
            position_y,
            size_x,
            size_y
        );

        self.objects.push(new_object);
    }

    pub fn output_display(&mut self) {
        self.repopulate_display();
        self.draw_objects_to_display();

        let mut output = String::new();

        for x in 0..self.size_x {
            for y in 0..self.size_y {
                match self.display[x][y] {
                    PixelType::Platform => { output = output + "#"; }
                    PixelType::Player => { output = output + "X"; }
                    PixelType::Empty => { output = output + " "; }
                }
            }
            output = output + "\n";
        }

        println!("OUTPUT: {output}");
    }

    fn bound_index(&self, x: usize, y: usize) -> (usize, usize) {
        let mut pair = (x, y);

        if x > self.size_x - 1 { pair.0 = self.size_x - 1 }
        if y > self.size_y - 1 { pair.1 = self.size_y - 1 }

        return pair;
    }

    fn draw_objects_to_display(&mut self) {
        for object in &self.objects {
            for x_offset in 0..object.size_x {
                for y_offset in 0..object.size_y {
                    let (x, y) = self.bound_index(
                        object.position_x + x_offset, 
                        object.position_y + y_offset
                    );

                    self.display[x][y] = object.pixel_type;
                }
            }
        }
    }

    fn repopulate_display(&mut self) {
        self.display.clear();

        for x in 0..self.size_x {
            self.display.push(Vec::new());
            for _ in 0..self.size_y {
                self.display[x].push(PixelType::Empty)
            }
        }


    }
}



/*
#[derive(Clone, Copy)]
pub enum TileType {
    Platform,
    Player,
    Empty
}


pub struct TileMap {
    tiles: Vec<Vec<TileType>>,
    size_x: usize,
    size_y: usize
}


impl TileMap {
    pub fn new(size_x: usize, size_y: usize) -> TileMap {
        let mut map = TileMap {
            tiles: Vec::new(),
            size_x: size_x,
            size_y: size_y
        };

        map.populate_map();

        return map;
    }

    pub fn draw_object(&mut self, tile_type: TileType, position_x: usize, position_y: usize, size_x: usize, size_y: usize) {
        for x_offset in 0..size_x {
            for y_offset in 0..size_y {
                let (x, y) = self.wrap_index(
                    position_x + x_offset, 
                    position_y + y_offset
                );

                self.tiles[x][y] = tile_type;
            }
        }
    }

    pub fn output_map(&self) {
        let mut output = String::new();

        for x in 0..self.size_x {
            for y in 0..self.size_y {
                match self.tiles[x][y] {
                    TileType::Platform => { output = output + "#" }
                    TileType::Player => { output = output + "X" }
                    TileType::Empty => { output = output + " " }
                }
            }
            output = output + "\n";
        }

        println!("{output}");
    }

    pub fn wrap_index(&self, x: usize, y: usize) -> (usize, usize) {
        let mut pair = (x, y);

        if x > self.size_x - 1 { pair.0 = 0 }
        if y > self.size_y - 1 { pair.1 = 0 }

        return pair;
    }

    fn populate_map(&mut self) {
        for x in 0..self.size_x {
            self.tiles.push(Vec::new());
            for _ in 0..self.size_y {
                self.tiles[x].push(TileType::Empty)
            }
        }
    }
}*/

/*use core::time;
use std::time::Duration;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum PixelType {
    Platform,
    Player,
    Empty
}


#[derive(Debug)]
struct DisplayObject {
    size_x: usize,
    size_y: usize,
    position_x: usize,
    position_y: usize,
    pixel_type: PixelType
}


impl DisplayObject {
    pub fn new(size_x: usize, size_y: usize, position_x: usize, position_y: usize, pixel_type: PixelType) -> DisplayObject {
        DisplayObject {
            size_x: size_x,
            size_y: size_y,
            position_x: position_x,
            position_y: position_y,
            pixel_type: pixel_type
        }
    }
}

#[derive(Debug)]
pub struct DisplayEngine {
    display_objects: Vec<DisplayObject>,
    pub display_pixels: Vec<Vec<PixelType>>,
    display_dimensions: (usize, usize),
    display_delay: Duration
}


impl DisplayEngine {
    pub fn new(display_size: (usize, usize), frame_time_miliseconds: u64) -> DisplayEngine {
        let mut engine = DisplayEngine {
            display_dimensions: display_size,
            display_objects: Vec::new(),
            display_pixels: Vec::new(),
            display_delay: time::Duration::from_millis(frame_time_miliseconds)
        };

        for x in 0..display_size.0 {
            for _ in 0..display_size.1 {
                engine.display_pixels[x].push(PixelType::Empty);
            }
        }

        return engine;
    }

    pub fn create_platform(&mut self, platform_size: (usize, usize), platform_position: (usize, usize)) {
        self.display_objects.push(DisplayObject::new(platform_size.0, platform_size.1, platform_position.0, platform_position.1, PixelType::Platform));
        self.update_display_without_output();
    }

    pub fn display_output(&self) {
        let mut output = String::new();

        for x in 0..self.display_dimensions.0 {
            for y in 0..self.display_dimensions.1 {
                match self.display_pixels[x][y] {
                    PixelType::Platform => { output = output + "#" }
                    PixelType::Player => { output = output + "X" }
                    PixelType::Empty => { output = output + " " }
                }
            }
        }

        println!("{output}");
    }

    fn index_wrap(&self, x: usize, y: usize) -> (usize, usize) {
        let mut pair = (x, y);

        if x > self.display_dimensions.0 - 1 { pair.0 = 0 }
        if y > self.display_dimensions.1 - 1 { pair.1 = 0 }

        return pair;
    }

    fn update_display_without_output(&mut self) {
        for display_object in &self.display_objects {
            for offset_x in 0..display_object.size_x {
                for offset_y in 0..display_object.size_y {
                    let (x, y) = self.index_wrap(display_object.position_x + offset_x, display_object.position_y + offset_y);

                    self.display_pixels[x][y] = display_object.pixel_type;
                }
            }
        }
    }

}*/


/*
use std::{thread, time::Duration};
use core::time;
 
enum TileType {
    Platform,
    Player,
    Empty
}

struct TileObject {
    tile_type: TileType,
}

impl TileObject {
    pub fn new(tile_type: TileType) -> TileObject {
        TileObject {
            tile_type: tile_type
        }
    }
}

pub struct DisplayEngine {
    frame_delay: Duration,
    display_size_x: usize,
    display_size_y: usize,
    display_grid: Vec<Vec<TileObject>>
}


impl DisplayEngine {
    pub fn new(display_size_x: usize, display_size_y: usize, frame_time_miliseconds: u64) -> DisplayEngine {
        let mut engine = DisplayEngine {
            frame_delay: time::Duration::from_millis(frame_time_miliseconds),
            display_size_x: display_size_x,
            display_size_y: display_size_y,
            display_grid: Vec::new()
        };

        engine.populate_display();

        return engine;
    }

    pub fn start_display_loop(&mut self) {
        loop {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            self.output_display();
            thread::sleep(self.frame_delay);
        }
    }

    pub fn create_platform(&mut self, size_x: usize, size_y: usize, position_x: usize, position_y: usize) {
        for offset_x in 0..size_x {
            for offset_y in 0..size_y {
                let (x, y) = self.wrap_index(position_x + offset_x, position_y + offset_y);

                self.display_grid[x][y].tile_type = TileType::Platform;
            }
        }
    }

    fn wrap_index(&self, x: usize, y: usize) -> (usize, usize) {
        let mut pair = (x, y);

        if x > self.display_size_x - 1 { pair.0 = 0; }
        if y > self.display_size_y - 1 { pair.1 = 0; }

        return pair;
    }

    fn output_display(&self) {
        let mut output = String::new();

        for x in 0..self.display_size_x {
            for y in 0..self.display_size_y {
                match self.display_grid[x][y].tile_type {
                    TileType::Platform => { output = output + "#" }
                    TileType::Player => { output = output + "X" }
                    TileType::Empty => { output = output + " " }
                }
            }
            output = output + "\n";
        }

        println!("{output}");
    }


    fn populate_display(&mut self) {
        for x in 0..self.display_size_x {
            self.display_grid.push(Vec::new());
            for _ in 0..self.display_size_y {
                self.display_grid[x].push(TileObject::new(TileType::Empty))
            }
        }
    }
}*/