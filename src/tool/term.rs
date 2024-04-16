use core::time;
use std::time::Duration;

enum PixelType {
    Platform,
    Player,
    Empty
}


struct DisplayObject {
    size_x: usize,
    size_y: usize,
    position_x: usize,
    position_y: usize
}


impl DisplayObject {
    pub fn new(size_x: usize, size_y: usize, position_x: usize, position_y: usize) -> DisplayObject {
        DisplayObject {
            size_x: size_x,
            size_y: size_y,
            position_x: position_x,
            position_y: position_y
        }
    }
}


pub struct DisplayEngine {
    display_objects: Vec<DisplayObject>,
    display_pixels: Vec<Vec<PixelType>>,
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
        self.display_objects.push(DisplayObject::new(platform_size.0, platform_size.1, platform_position.0, platform_position.1))
    }


    fn update_display_without_output(&mut self) {
        for display_object in &self.display_objects {
            
        }
    }

}


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