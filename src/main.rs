mod maze_algorithms;
mod util;
mod tests;
mod pathfinding;

use num_format::{Locale, ToFormattedString};
use util::args::parse_args;
use util::flush_out;

fn main() {
    println!("Generating maze object...");
    let mut maze = maze_algorithms::iterative::Engine::new(500, 500);
    println!("Done\n");

    println!("Generating maze...");
    maze.run();
    println!("Done\n");

    println!("Saving image...");
    maze.save_image("tmp.png".to_string(), 1).unwrap();
    println!("Done\n");

    println!("Calculating path...");
    pathfinding::calculate_path("./tmp.png");
    println!("Done\n");

    /*
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    };


    let now = std::time::Instant::now();

    println!("Creating maze object...");
    let mut iter_maze = maze_algorithms::iterative::Engine::new(args.x, args.y);

    println!("Generating maze...");
    iter_maze.run();
    println!("Done, time taken for {} ({}x{}) elements: {}ms", iter_maze.num_elements().to_formatted_string(&Locale::de), iter_maze.size.x.to_formatted_string(&Locale::de), iter_maze.size.y.to_formatted_string(&Locale::de), now.elapsed().as_millis().to_formatted_string(&Locale::de));

    println!("\nSaving image...");
    let now = std::time::Instant::now();
    iter_maze.save_image(args.filepath, args.pixel_size).unwrap();
    println!("Done, took {}ms", now.elapsed().as_millis().to_formatted_string(&Locale::de));
     */
}