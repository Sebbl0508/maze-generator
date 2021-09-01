use clap::{App, Arg, SubCommand};

#[derive(Default, Debug)]
pub struct Args {
    pub x: u32,
    pub y: u32,
    pub pixel_size: u32,
    pub filepath: String,
    pub save_cells: bool,
}

pub fn parse_args() -> Result<Args, String> {
    let matches = App::new("Maze-rs")
        .version("1.1")
        .author("sebbl0508")
        .about("A maze generator and solver written in Rust")
        .subcommand(SubCommand::with_name("gen")
            .about("Generates the maze")
            .arg(Arg::with_name("X")
                .value_name("X")
                .help("The X size of the maze")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("Y")
                .value_name("Y")
                .help("The Y size of the maze")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("CELLSIZE")
                .short("c")
                .long("cellsize")
                .help("Sets the size of a Cell in pixels")
                .takes_value(true)
                .required(false)
                .default_value("20"))
            .arg(Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .help("The path where the image should be saved, if not specified, will not be saved")
                .required(false)
                .takes_value(true))
            .arg(Arg::with_name("SAVECELLS")
                .short("s")
                .long("save")
                .help("Saves the grid struct in a JSON file for using in Maze-solving")
                .takes_value(false))
            )
        .subcommand(SubCommand::with_name("solve")
            .about("Takes a maze in JSON format, solves it and saves the picture")
            .arg(Arg::with_name("INPUT")
                .value_name("INPUT")
                .help("The path of the JSON file")
                .required(true))
            .arg(Arg::with_name("OUTPUT")
                .value_name("OUTPUT")
                .help("The output path of the Picture file")
                .required(true)
            )
        ).get_matches();

    Ok(Args {
        x: 25,
        y: 25,
        pixel_size: 20,
        filepath: "./output.png".to_string(),
        save_cells: false,
    })
}

/*
pub fn parse_args_old() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(format!("Too few arguments!\nUsage: './{} [X] [Y] [pixel size (optional)] [output filepath (optional)]'", args[0]));
    }

    let x = match args[1].parse() {
        Ok(v) => v,
        Err(_) => {
            return Err("Argument [X] can only be a number".to_string());
        }
    };

    let y = match args[2].parse() {
        Ok(v) => v,
        Err(_) => {
            return Err("Argument [Y] can only be a number".to_string());
        }
    };




    if args.len() == 3 {
        return Ok(Args {
            x,
            y,
            pixel_size: 20,
            filepath: "output.png".to_string()
        });
    } else if args.len() == 4 {
        let pxl_size = match args[3].parse() {
            Ok(v) => v,
            Err(_) => {
                return Err("Argument [pixel size] can only be a number".to_string());
            }
        };

        return Ok(Args {
            x,
            y,
            pixel_size: pxl_size,
            filepath: "output.png".to_string()
        })
    } else if args.len() == 5 {
        let pxl_size = match args[3].parse() {
            Ok(v) => v,
            Err(_) => {
                return Err("Argument [pixel size] can only be a number".to_string());
            }
        };

        let filepath = args[4].clone();

        return Ok(Args {
            x,
            y,
            pixel_size: pxl_size,
            filepath,
        })
    }

    return Err("This shouldn't happen!".to_string());
}
*/
