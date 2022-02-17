mod test;

use std::error::Error;
use std::io;

/*
   author: jinshikai
   create date: 2/16/22
   desc: 运行函数主体
*/
pub fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            return Ok(());
        }
        println!("{}", input.trim());
        input.clear();
    }
}
