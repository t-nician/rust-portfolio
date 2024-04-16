const DISPLAY_SIZE_X: usize = 10;
const DISPLAY_SIZE_Y: usize = 10;

enum TileType {
    Player,
    Platform,
    Empty
}

struct Collision {
    size_x: usize,
    size_y: usize,
    position_x: usize,
    position_y: usize
}

impl Collision {
    pub fn new(size_x: usize, size_y: usize, position_x: usize, position_y: usize) -> Collision {
        Collision {
            size_x: size_x,
            size_y: size_y,
            position_x: position_x,
            position_y: position_y
        }
    }
}


struct Platform {
    collision: Collision
}


impl Platform {
    pub fn new(size_x: usize, size_y: usize, position_x: usize, position_y: usize) -> Platform {
        Platform {
            collision: Collision::new(
                size_x,
                size_y,
                position_x,
                position_y
            )
        }
    }
}


struct Player {
    collision: Collision
}


impl Player {
    pub fn new(size_x: usize, size_y: usize, position_x: usize, position_y: usize)  -> Player {
        Player {
            collision: Collision::new(
                size_x,
                size_y,
                position_x,
                position_y
            )
        }
    }
}


pub struct JumpGame {
    platforms: Vec<Platform>,
    display: Vec<Vec<TileType>>,
    player: Player,
}


impl JumpGame {
    pub fn new() -> JumpGame {
        let mut game = JumpGame {
            display: Vec::new(),
            platforms: Vec::new(),
            player: Player::new(2, 2, 5, 5)
        };

        game.populate_display();
        
        game.platforms.push(
            Platform::new(10, 1, 1, 8)
        );

        return game;
    }

    pub fn update_and_display(&mut self) {
        print!("\x1B[2J\x1B[1;1H");
        
        let mut display_string = String::new();

        self.update_collisions_to_display();

        for x in 0..DISPLAY_SIZE_X {
            for y in 0..DISPLAY_SIZE_Y {
                match self.display[x][y] {
                    TileType::Platform => { display_string = display_string + "X" }
                    TileType::Player => { display_string = display_string + "O" }
                    TileType::Empty => { display_string = display_string + " " }
                }
            }

            display_string = display_string + "\n";
        }

        println!("{display_string}");

    }

    fn wrap_index(x: i64, y: i64) -> (usize, usize) {
        let mut new_position: (usize, usize) = (0, 0);

        if x < 0 { new_position.0 = DISPLAY_SIZE_X - 1 } else { new_position.0 = x as usize; }
        if x > DISPLAY_SIZE_X as i64 { new_position.0 = DISPLAY_SIZE_X - 1 }

        if y < 0 { new_position.1 = DISPLAY_SIZE_Y - 1 } else { new_position.1 = y as usize; }
        if y > DISPLAY_SIZE_Y as i64 { new_position.1 = DISPLAY_SIZE_Y - 1 }

        return new_position;
    }

    fn update_collisions_to_display(&mut self) {
        for x in 0..DISPLAY_SIZE_X {
            for y in 0..DISPLAY_SIZE_Y {
                self.display[x][y] = TileType::Empty;
            }
        }

        for platform in &self.platforms {
            for x in 0..platform.collision.size_x as i64 {
                for y in 0..platform.collision.size_y as i64 {
                    
                }
            }
        }
    }

    fn populate_display(&mut self) {
        for x in 0..DISPLAY_SIZE_X {
            self.display.push(Vec::new());
            for _ in 0..DISPLAY_SIZE_Y {
                self.display[x].push(TileType::Empty);
            }
        }
    }
}