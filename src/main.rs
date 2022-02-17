use std::process;

fn main() {
    if let Err(e) = jloc::run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
