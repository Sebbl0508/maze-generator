mod algorithms;
mod util;
use num_format::{Locale, ToFormattedString};
use util::args::parse_args;

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    };


    let now = std::time::Instant::now();
    let mut iter_maze = algorithms::iterative::Engine::new(args.x, args.y);

    println!("Generating maze...");
    iter_maze.run();
    println!("Done, time taken for {} ({}x{}) elements: {}ms", iter_maze.num_elements().to_formatted_string(&Locale::de), iter_maze.size.x.to_formatted_string(&Locale::de), iter_maze.size.y.to_formatted_string(&Locale::de), now.elapsed().as_millis().to_formatted_string(&Locale::de));

    println!("\nSaving image...");
    let now = std::time::Instant::now();
    iter_maze.save_image(args.filepath, args.pixel_size).unwrap();
    println!("Done, took {}ms", now.elapsed().as_millis().to_formatted_string(&Locale::de));
}
