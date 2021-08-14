mod image_generation;
mod algorithms;
mod util;

fn main() {
    let mut recursive_maze = algorithms::recursive::Pointer::new(500, 500);
    let mut iter_maze = algorithms::iterative::Engine::new(500, 500);

    println!("Generating maze...");
    iter_maze.run();
    println!("Done");

    use std::fs::File;
    use std::io::prelude::*;

//    let mut my_file = File::create("Debug_out.txt").unwrap();
//    let mut my_buf = format!("{:#?}", [MAZE_STRUCT_HERE]);
//    let _ = my_file.write_all(my_buf.as_bytes()).unwrap();
}
