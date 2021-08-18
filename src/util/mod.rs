use std::io::Write;

pub mod direction;
pub mod vec2;
pub mod args;


// Others
pub fn flush_out() {
    let mut tmp = std::io::stdout();
    let _ = tmp.flush().unwrap();
}