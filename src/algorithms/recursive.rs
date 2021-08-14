#[derive(Debug, Clone)]
pub struct Cell {
    x: i64,
    y: i64,
    visited: bool,
    dir_walked: Option<Direction>
}

#[derive(Debug, Clone)]
pub struct Pointer {
    x: usize,
    y: usize,
    pub grid: Vec<Vec<Cell>>,
    grid_shape: (i64, i64)
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl Pointer {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: 0,
            y: 0,
            grid: Self::build_grid(x, y),
            grid_shape: (x as i64, y as i64),
        }
    }

    pub fn build_grid(x: usize, y: usize) -> Vec<Vec<Cell>> {
        let mut my_grid: Vec<Vec<Cell>> = Vec::new();

        for iter_x in 0..x {
            let mut tmp_x: Vec<Cell> = Vec::new();
            for iter_y in 0..y {
                tmp_x.push(Cell::new(iter_x as i64, iter_y as i64));
            }

            my_grid.push(tmp_x);
        }

        return my_grid;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        let tmp = match self.grid.get(x) {
            Some(v) => {
                match v.get(y) {
                    Some(v) => v,
                    None => {
                        return None;
                    }
                }
            },
            None => {
                return None;
            }
        };

        // Maybe pass mutable reference?
        return Some(tmp.clone());
    }

    fn recurse(&mut self, input: &mut Cell) {
        while input.has_unvisited_neighbors(self).is_some() {
            let mut sel_neighbor = input.has_unvisited_neighbors(self).unwrap();
            let mut sel_neighbor = self.grid[sel_neighbor.x as usize][sel_neighbor.y as usize].clone();


            self.recurse(&mut sel_neighbor);
        }
    }

    pub fn run(&mut self) {
        let mut curr_cell = self.grid[self.x][self.y].clone();
        self.recurse(&mut curr_cell);

        // while curr_cell.has_unvisited_neighbors(&mut self).is_some() {
        //     let sel_neighbor = curr_cell.has_unvisited_neighbors(&mut self).unwrap();
        // }
    }
}

impl Cell {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
            visited: false,
            dir_walked: None,
        }
    }

    pub fn visit(&mut self) {
        if self.visited {
            panic!("This should NOT happen :/\nThe cell was already visited");
        } else {
            self.visited = true;
        }
    }

    pub fn has_unvisited_neighbors(&mut self, grid: &mut Pointer) -> Option<Cell> {
        let dirs = ((self.x, self.y-1), (self.x, self.y+1), (self.x-1, self.y), (self.x+1, self.y));
        let neighbors = vec![
            grid.get_cell(dirs.0.0 as usize, dirs.0.1 as usize),
            grid.get_cell(dirs.1.0 as usize, dirs.1.1 as usize),
            grid.get_cell(dirs.2.0 as usize, dirs.2.1 as usize),
            grid.get_cell(dirs.3.0 as usize, dirs.3.1 as usize)
            ];

        for (i, this) in neighbors.iter().enumerate() {
            if this.is_some() {
                let mut tmp = this.clone().unwrap();

                match i {
                    0 => {
                        self.dir_walked = Some(Direction::Down);
                    }
                    1 => {
                        self.dir_walked = Some(Direction::Up);
                    }
                    2 => {
                        self.dir_walked = Some(Direction::Left);
                    }
                    3 => {
                        self.dir_walked = Some(Direction::Right);
                    }
                    _ => {
                        panic!("WTF??");
                    }
                }

                return Some(tmp);
            }
        }

        return None;
    }
}

/*
// Implement default stuff
#[derive(Debug, Clone)]
pub struct Pointer {
    x: i64,
    y: i64,
    stack: Vec<(i64, i64)>,
    shape: (u64, u64),
    grid: Vec<Vec<i64>>,
    locations: Vec<(i64, i64)>,
}

impl Pointer {
    pub fn new(grid_size: (u64, u64)) -> Self {
        Self {
            x: 0,
            y: 0,
            stack: Vec::new(),
            shape: grid_size,
            grid: vec![vec![0; grid_size.1 as usize]; grid_size.0 as usize],
            locations: Vec::new(),
        }
    }

    pub fn run_check(&mut self) -> bool {
        println!("{}", self.shape.0 * self.shape.1 / (self.locations.len() + 1) as u64);
        if self.shape.0 * self.shape.1 / (self.locations.len() + 1) as u64 == 1 {
            return false;
        }
        return true;
    }

    pub fn get_free_neighbor(&mut self) {

    }
}
 */