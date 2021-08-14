mod image_generation;
mod algorithms;
mod util;
use serde_json::to_string_pretty;
use num_format::{Locale, ToFormattedString};

fn main() {
    let now = std::time::Instant::now();
    let mut iter_maze = algorithms::iterative::Engine::new(100, 100);

    println!("Generating maze...");
    iter_maze.run();
    println!("Done, time taken for {} ({}x{}) elements: {}ms", iter_maze.num_elements().to_formatted_string(&Locale::de), iter_maze.size.x.to_formatted_string(&Locale::de), iter_maze.size.y.to_formatted_string(&Locale::de), now.elapsed().as_millis().to_formatted_string(&Locale::de));
}
