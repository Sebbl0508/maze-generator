use rand::prelude::*;
use crate::util::{
    vec2::Vec2,
    direction::Direction,
};

#[derive(Debug, Clone)]
pub struct Cell {
    pos: Vec2<u32>,
    dir_walked: Option<Direction>,
    visited: bool,
}

#[derive(Debug, Clone)]
pub struct Engine {
    stack: Vec<Vec2<usize>>,
    pub size: Vec2<u32>,
    grid: Vec<Vec<Cell>>,
    visited_places: u64
}

impl Engine {
    /// Contructor for the Engine Struct
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            stack: Vec::new(),
            size: Vec2::new(x, y),
            grid: Self::generate_grid(x, y),
            visited_places: 0,
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
                    dir_walked: None,
                    visited: false,
                })
            }

            tmp.push(tmp_x);
        }

        return tmp;
    }

    pub fn print_progress(&self) {
        #[cfg(debug_assertions)]
        println!("{:#?}", (self.size.x * self.size.y) as f64 / (self.visited_places + 1) as f64);
    }

    /// Number of elements in the grid
    pub fn num_elements(&self) -> i64 {
        self.size.x as i64 * self.size.y as i64
    }

    /// This method starts the generation algorithm
    pub fn run(&mut self) {
        let mut rng = thread_rng();
        let start_pos: Vec2<u32> = Vec2::new(rng.gen_range(0..self.size.x), rng.gen_range(0..self.size.y));
        self.stack.push(Vec2::new(start_pos.x as usize, start_pos.y as usize));

        while !self.stack.is_empty() {
            // TODO: This progress number should stop a 1.0, but it continues < 1.0
            // Figure out why :/
            self.print_progress();

            // First get the coordinate of the current cell off the stack
            // Then shadow the coord variable and put the a mutable reference to the actual cell into the variable
            let curr_cell = self.stack.pop().unwrap();
//            let curr_cell = self.mut_cell(curr_cell.x, curr_cell.y);

            match self.unvisited_neighbors(curr_cell.clone()) {
                Some(v) => {
                    self.stack.push(curr_cell.clone());
                    self.stack.push(v);

                    self.visited_places += 1;
//                    let actual_cell = self.mut_cell(curr_cell.x, curr_cell.y);
                }
                None => {
                    continue;
                }
            }

        }
    }

    /// Just get a mutable reference to a single cell by coordinate values
    pub fn mut_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        self.grid.get_mut(x).unwrap().get_mut(y).unwrap()
    }

    /// Like `mut_cell` but returns `None` if the cell doesn't exist
    pub fn mut_cell_safe(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        match self.grid.get_mut(x) {
            Some(v) => {
                match v.get_mut(y) {
                    Some(v) => Some(v),
                    None => {
                        return None;
                    }
                }
            }
            None => {
                return None
            }
        }
    }

    /// Looks for not visited neighbors of the cell at the supplied coordinate
    ///
    /// If there are only already visited cells as neighbors,
    /// this will return `None`
    ///
    ///
    /// If there were not-visited neighboring cells,
    /// one will be randomly chosen it's coordinates returned
    pub fn unvisited_neighbors(&mut self, cell_coord: Vec2<usize>) -> Option<Vec2<usize>> {
        let mut rng = thread_rng();
        let mut tmp_vec: Vec<Direction> = Vec::new();
        self.mut_cell(cell_coord.x, cell_coord.y).visited = true;

        if cell_coord.x > 0 {
            let left_neighbor = cell_coord.clone() + Direction::Left.vector();
            let actual_cell = self.mut_cell_safe(left_neighbor.x, left_neighbor.y);

            if actual_cell.is_some() && !actual_cell.unwrap().visited() {
                tmp_vec.push(Direction::Left);
            }
        }

        if cell_coord.x < (self.size.x - 1) as usize {
            let right_neighbor = cell_coord.clone() + Direction::Right.vector();
            let actual_cell = self.mut_cell_safe(right_neighbor.x, right_neighbor.y);

            if actual_cell.is_some() && !actual_cell.unwrap().visited() {
                tmp_vec.push(Direction::Right);
            }
        }

        if cell_coord.y > 0 {
            // Y = 0 Is the bottom, y+1 goes up
            let bottom_neighbor = cell_coord.clone() + Direction::Down.vector();
            let actual_cell = self.mut_cell_safe(bottom_neighbor.x, bottom_neighbor.y);

            if actual_cell.is_some() && !actual_cell.unwrap().visited() {
                tmp_vec.push(Direction::Down);
            }
        }

        if cell_coord.y < (self.size.y - 1) as usize {
            let top_neighbor = cell_coord.clone() + Direction::Up.vector();
            let actual_cell = self.mut_cell_safe(top_neighbor.x, top_neighbor.y);
            if actual_cell.is_some() && !actual_cell.unwrap().visited() {
                tmp_vec.push(Direction::Up);
            }
        }

        if tmp_vec.is_empty() {
            return None;
        }



        // Get a random neighbor and return
        let rand_dir = tmp_vec.choose(&mut rng).unwrap().clone();
        let rand_vec = cell_coord.clone() + rand_dir.vector();

        let to_cell = self.mut_cell(rand_vec.x, rand_vec.y);
        to_cell.visited = true;
        self.mut_cell(cell_coord.x, cell_coord.y).dir_walked = Some(rand_dir);

        return Some(rand_vec);
    }

    pub fn save_image(&self, filepath: String, pixel_size: u32) -> Result<(), String> {
        img_export::save_image(self, pixel_size as i32, filepath);

        Ok(())
    }
}

impl Cell {
    /// Constructor for the Cell Structure
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            dir_walked: None,
            visited: false,
        }
    }

    /// Returns if the cell was visited before
    pub fn visited(&self) -> bool {
        // If the dir_walked attribute is set, then the cell must have been visited
        return self.visited;
    }
}

mod img_export {
    use crate::util::vec2::Vec2;
    use crate::util::direction::Direction;
    use image::{ImageBuffer, ImageFormat, Rgb};
    use imageproc::{
        rect::{Rect, RectPosition},
        drawing,
    };


    pub fn save_image(state: &super::Engine, pixel_size: i32, filepath: String) {
        let size = Vec2::new(state.size.y as i32 * pixel_size * 2 - pixel_size, state.size.x as i32 * pixel_size * 2 - pixel_size);
        println!("Pic size: {}x{}", size.x, size.y);

        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(size.x as u32, size.y as u32);

        // Fill background White
        drawing::draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(size.x as u32, size.y as u32), Rgb([255, 255, 255]));

        // Draw black every odd row
        for y in 0..(state.size.x * 2) {
            if y % 2 == 1 {
                let tmp = Rect::at(0, y as i32 * pixel_size)
                    .of_size(size.x as u32, pixel_size as u32);

                drawing::draw_filled_rect_mut(&mut img, tmp, Rgb([0, 0, 0]));
            }
        }

        // Same for columns
        for x in 0..(state.size.y * 2) {
            if x % 2 == 1 {
                let tmp = Rect::at(x as i32 * pixel_size, 0)
                    .of_size(pixel_size as u32, size.y as u32);

                drawing::draw_filled_rect_mut(&mut img, tmp, Rgb([0, 0, 0]));
            }
        }
        let mut skipped = 0;

        for x in 0..(state.size.x) as usize {
            for y in 0..(state.size.y) as usize {
                let item = &state.grid[x][y];

                let yy = y * 2;
                let xx = x * 2;

                if !item.visited || item.dir_walked.is_none() {
                    skipped += 1;
                    continue;
                }

                let mut rect = Rect::at(0, 0).of_size(1, 1);

                match item.dir_walked.unwrap() {
                    Direction::Up => {
                        rect = Rect::at(xx as i32 * pixel_size, yy as i32 * pixel_size - pixel_size).of_size(pixel_size as u32, pixel_size as u32);
                    }
                    Direction::Down => {
                        rect = Rect::at(xx as i32 * pixel_size, yy as i32 * pixel_size + pixel_size).of_size(pixel_size as u32, pixel_size as u32);
                    }
                    Direction::Left => {
                        rect = Rect::at(xx as i32 * pixel_size - pixel_size, yy as i32 * pixel_size).of_size(pixel_size as u32, pixel_size as u32);
                    }
                    Direction::Right => {
                        rect = Rect::at(xx as i32 * pixel_size + pixel_size, yy as i32 * pixel_size).of_size(pixel_size as u32, pixel_size as u32);
                    }
                }
                drawing::draw_filled_rect_mut(&mut img, rect, Rgb([255, 255, 255]));
            }
        }

        println!("Skipped {} Cells", skipped);


        img.save(filepath).unwrap();
    }
}