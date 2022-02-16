use std::error::Error;
use std::{fs, io};

/*
   file: src/lib.rs
   author: jinshikai
   time: 22.2.16
   desc: 命令行参数数据结构
*/
pub struct Config {
    pub filename: String,
}

/*
   file: src/lib.rs
   author: jinshikai
   time: 22.2.16
   desc: 命令行参数行为
*/
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

/*
   file: src/lib.rs
   author: jinshikai
   time: 22.2.16
   desc: 运行主体函数
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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
