use crate::util::vec2::Vec2;
use crate::util::direction::Direction;

use image::{Rgb, ImageBuffer, RgbImage, ImageFormat, io::Reader as ImgReader, DynamicImage, Pixel, Rgba, GenericImageView, GenericImage};
use imageproc::{
    rect::{Rect, RectPosition},
    drawing,
};
use std::path::Path;

use rand::prelude::*;
// use imageproc::drawing::Canvas;

#[derive(Debug, Clone)]
struct Engine {
    field: Field,
}

#[derive(Debug, Clone)]
struct Field {
    size: Vec2<u32>,
    grid: Vec<Vec<Option<Cell>>>,
}

#[derive(Debug, Clone)]
struct Cell {
    x: u32,
    y: u32,
    kind: CellType,
    visited: bool,
    distance: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum CellType {
    Wall,
    Space,
}

pub fn load_img(img_path: &str) -> DynamicImage {
    let img_path = Path::new(img_path);

    return ImgReader::open(img_path).unwrap().decode().unwrap();
}

pub fn calculate_path(img_path: &str) {
    let mut image = load_img(img_path);

    // Get image information
    let img_dimensions = image.dimensions();
    let begin_point = Vec2::new(0, 0);
    let end_point = Vec2::new(img_dimensions.0-1, img_dimensions.1-1);
    let mut solver = Engine::new(Vec2::new(img_dimensions.0, img_dimensions.1));

    // Initialize the field
    for x in 0..img_dimensions.0 {
        for y in 0..img_dimensions.1 {
            let pxl = image.get_pixel(x, y);
            // Just unwrap, if unwrap fails something went wrong
            let cur_cell = solver.field.mut_cell_safe(x, y);

            let default_dist = if x == 0 && y == 0 {
                0
            } else {
                u64::MAX
            };

            // Wall black pixel
            if pxl.0[0] == 0x00 {
                solver.field.grid[x as usize][y as usize] = Some(Cell::new(x, y, CellType::Wall, default_dist));
            } else if pxl.0[0] == 0xFF { // White free pixel
                solver.field.grid[x as usize][y as usize] = Some(Cell::new(x, y, CellType::Space, default_dist));
            } else {
                panic!("This should NOT happen!");
            }
        }
    }

    let mut rng = thread_rng();

    let mut cur = solver.field.cell_safe(0, 0).unwrap();



    // Debug print
    // println!("{:#?}", solver.field.grid);

    image.save("tmp_solved.png").unwrap();
}

impl Field {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            size: Vec2 {x, y},
            grid: vec![vec![None; y as usize]; x as usize],
        }
    }

    pub fn cell_safe(&self, x: u32, y: u32) -> Option<Cell> {
        match self.grid.get(x as usize) {
            Some(v) => {
                match v.get(y as usize) {
                    Some(v) => v.clone(),
                    None => None,
                }
            },
            None => None
        }
    }

    pub fn valid_neighbors(&self, x: u32, y: u32) -> Vec<Cell> {
        let mut valids: Vec<Cell> = Vec::new();

        // Get the vectors of all directions
        let directions: Vec<(Direction, Vec2<isize>)> = Direction::all()
            .to_vec()
            .iter()
            .map(|v| (v.clone(), v.vector()))
            .collect();

        for i in directions {
            let other = Vec2::new(x as isize, y as isize) + i.1;

            let cur = self.cell_safe(other.x as u32, other.y as u32);
            if let Some(v) = cur {
                if v.kind != CellType::Wall && v.visited == false {
                    valids.push(v);
                }
            }
        }

        return valids;
    }

    pub fn mut_cell_safe(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        match self.grid.get_mut(x as usize) {
            Some(v) => {
                match v.get_mut(y as usize) {
                    Some(v) => {
                        match v {
                            Some(v) => Some(v),
                            None => None,
                        }
                    },
                    None => None
                }
            },
            None => None,
        }
    }
}

impl Cell {
    pub fn new(x: u32, y: u32, celltype: CellType, distance: u64) -> Self {
        Cell {
            x,
            y,
            kind: celltype,
            visited: false,
            distance,
        }
    }
}

impl Engine {
    pub fn new(field_size: Vec2<u32>) -> Self {
        Self {
            field: Field::new(field_size.x, field_size.y),
        }
    }
}