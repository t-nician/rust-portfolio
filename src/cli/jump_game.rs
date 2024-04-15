const DISPLAY_SIZE_X: usize = 10;
const DISPLAY_SIZE_Y: usize = 10;

enum TileType {
    Player,
    Platform,
    Empty
}

struct Collision {
    size_x: usize,
    size_y: usize
}

impl Collision {
    pub fn new(size_x: usize, size_y: usize) -> Collision {
        Collision {
            size_x: size_x,
            size_y: size_y
        }
    }
}


struct Platform {
    collision: Collision
}


impl Platform {
    pub fn new(size_x: usize, size_y: usize) -> Platform {
        Platform {
            collision: Collision::new(
                size_x,
                size_y
            )
        }
    }
}


struct Player {
    collision: Collision
}


impl Player {
    pub fn new(size_x: usize, size_y: usize) -> Player {
        Player {
            collision: Collision::new(
                size_x,
                size_y
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
            player: Player::new(2, 2)
        };

        game.populate_display();
        

        return game;
    }

    pub fn update_and_display(&mut self) {
        print!("\x1B[2J\x1B[1;1H");
        
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