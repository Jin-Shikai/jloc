use jloc::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    if let Err(e) = jloc::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
