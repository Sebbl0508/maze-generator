use std::env;

pub struct Args {
    pub x: u32,
    pub y: u32,
    pub pixel_size: u32,
    pub filepath: String,
}

pub fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(format!("Too few arguments!\nUsage: ./{} [X] [Y] [pixel size (optional)] [output filepath (optional)]", args[0]));
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

