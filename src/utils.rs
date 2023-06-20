use crate::config::{IP, PORT, VERSION};

pub fn print_info() {
    println!();
    println!("* Server v{}", VERSION);
    println!("* http://{}:{}", IP, PORT);
}
