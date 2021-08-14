use rand::prelude::*;
use crate::util::{
    vec2::Vec2,
    direction::Direction,
};

#[derive(Debug, Clone)]
pub struct Cell {
    pos: Vec2<u32>,
    dir_walked: Option<Direction>,
}

#[derive(Debug, Clone)]
pub struct Engine {
    stack: Vec<Vec2<usize>>,
    size: Vec2<u32>,
    grid: Vec<Vec<Cell>>,
}

impl Engine {
    /// Contructor for the Engine Struct
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            stack: Vec::new(),
            size: Vec2::new(x, y),
            grid: Self::generate_grid(x, y),
        }
    }

    /// Generates a 2x2 Grid of Cells by size x/y
    pub fn generate_grid(x: u32, y: u32) -> Vec<Vec<Cell>> {
        let mut tmp: Vec<Vec<Cell>> = Vec::new();

        for iter_x in 0..x {
            let mut tmp_x: Vec<Cell> = Vec::new();

            for iter_y in 0..y {
                tmp_x.push(Cell {
                    pos: Vec2::new(iter_x, iter_y),
                    dir_walked: None
                })
            }

            tmp.push(tmp_x);
        }

        return tmp;
    }

    /// This method starts the generation algorithm
    pub fn run(&mut self) {
        let mut rng = thread_rng();
        let start_pos: Vec2<u32> = Vec2::new(rng.gen_range(0..self.size.x), rng.gen_range(0..self.size.y));
        self.stack.push(Vec2::new(start_pos.x as usize, start_pos.y as usize));

        while !self.stack.is_empty() {
            // First get the coordinate of the current cell off the stack
            // Then shadow the coord variable and put the a mutable reference to the actual cell into the variable
            let curr_cell = self.stack.pop().unwrap();
            let curr_cell = self.mut_cell(curr_cell.x, curr_cell.y);


        }
    }

    /// Just get a mutable reference to a single cell by coordinate values
    pub fn mut_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        self.grid.get_mut(x).unwrap().get_mut(y).unwrap()
    }

    /// Looks for not visited neighbors of the cell at the supplied coordinate
    ///
    /// If there are only already visited cells as neighbors,
    /// this will return `None`
    ///
    ///
    /// If there were not visited neighboring cells
    /// their coordinates will be returned in a `Vec`
    pub fn unvisited_neighbors(&mut self, cell_coord: Vec2<usize>) -> Option<Vec<Vec2<usize>>> {
        let mut tmp_vec: Vec<Vec2<usize>> = Vec::new();

        if cell_coord.x > 0 {
            tmp_vec.push((cell_coord.clone() + Direction::Left.vector()));
        }

        if cell_coord.x < (self.size.x - 1) as usize {
            tmp_vec.push((cell_coord.clone() + Direction::Right.vector()));
        }

        return Some(tmp_vec);
    }
}

impl Cell {
    /// Constructor for the Cell Structure
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            dir_walked: None,
        }
    }

    /// Returns if the cell was visited before
    pub fn visited(&self) -> bool {
        // If the dir_walked attribute is set, then the cell must have been visited
        return self.dir_walked.is_some();
    }
}