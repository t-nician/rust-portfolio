use rand::Rng;



static GRID_SIZE_X: i64 = 10;
static GRID_SIZE_Y: i64 = 50;


#[derive(PartialEq)]
enum CellState {
    Alive,
    Dead
}


struct Cell {
    state: CellState
}


impl Cell {
    pub fn new(state: CellState) -> Cell {
        Cell {
            state: state
        }
    }
}


pub struct ConwaysGame {
    grid: Vec<Vec<Cell>>,

    grid_size_x: i64,
    grid_size_y: i64
}


impl ConwaysGame {
    pub fn new() -> ConwaysGame {
        let mut game = ConwaysGame {
            grid: Vec::new(),
            grid_size_x: GRID_SIZE_X,
            grid_size_y: GRID_SIZE_Y
        };

        game.populate_grid();

        return game;
    }

    
    pub fn display_grid(&self) {
        let mut display_string = String::new();

        for x in 0..self.grid_size_x {
            for y in 0..self.grid_size_y {
                match self.grid[x as usize][y as usize].state {
                    CellState::Alive => { display_string = display_string + "X" }
                    CellState::Dead => { display_string = display_string + " " }
                }
            }
            display_string = display_string + "\n";
        }

        println!("{display_string}");
    }


    pub fn game_step(&mut self) {
        let mut tasks: Vec<(usize, usize, CellState)> = Vec::new();

        self.count_neighbors(5, 5);

        for x in 0..self.grid_size_x {
            for y in 0..self.grid_size_y {
                match self.grid[x as usize][y as usize].state {
                    CellState::Alive => {
                        let neighbors = self.count_neighbors(x as usize, y as usize);

                        if neighbors > 3 || neighbors < 2 {
                            tasks.push((x as usize, y as usize, CellState::Dead))
                        }
                    }

                    CellState::Dead => {
                        let neighbors = self.count_neighbors(x as usize, y as usize);

                        if neighbors == 3 {
                            tasks.push((x as usize, y as usize, CellState::Alive))
                        }
                    }
                }
            }
        }

        for item in tasks {
            let (x, y, state) = item;

            self.grid[x][y].state = state;
        }
    }


    fn wrap_index_bounds(&self, x: i64, y: i64) -> (i64, i64) {

        let mut result: (i64, i64) = (x, y);

        if x < 0 { result.0 = self.grid_size_x - 1; }
        if x > self.grid_size_x - 1 as i64 { result.0 = 0 }

        if y < 0 { result.1 = self.grid_size_y - 1; }
        if y > self.grid_size_y - 1 as i64 { result.1 = 0 }

        return result;
    }

    
    pub fn count_neighbors(&mut self, core_x: usize, core_y: usize) -> i64 {
        let mut count = if self.grid[core_x][core_y].state == CellState::Alive { -1 } else { 0 };

        for offset_x in -1..2 as i64 {
            for offset_y in -1..2 as i64 {
                let (target_x, target_y) = self.wrap_index_bounds(core_x as i64 + offset_x, core_y as i64 + offset_y);

                match self.grid[target_x as usize][target_y as usize].state {
                    CellState::Alive => { count += 1 }
                    _ => {}
                }
            }
        }

        return count;
    }


    fn populate_grid(&mut self) {
        for x in 0..self.grid_size_x {
            self.grid.push(Vec::new());
            for _ in 0..self.grid_size_y {
                match rand::thread_rng().gen_range(0..2) {
                    0 => { self.grid[x as usize].push(Cell::new(CellState::Alive)); },
                    1 => { self.grid[x as usize].push(Cell::new(CellState::Dead)); },
                    _ => { }
                }
            }
        }
    }
}