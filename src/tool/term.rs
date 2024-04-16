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
}