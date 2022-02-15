use std::{env, process};

use jloc::Config;

use std::io;
fn main() {
    let mut input = String::new();
    loop {
        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Error:{}", e);
            process::exit(1);
        }
        if input.is_empty() {
            process::exit(0);
        }
        println!("{}", input.trim());
        input.clear();
    }
}
