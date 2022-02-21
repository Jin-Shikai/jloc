use jloc::Config;
use std::env;
use std::process;

fn main() {
    if let Err(e) = jloc::run() {
        let args: Vec<String> = env::args().collect();
        let config = Config::new(&args);
        println!("Application error: {}", e);
        process::exit(1);
    }
}
