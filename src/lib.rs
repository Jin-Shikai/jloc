mod test;

use serde_json::Value;
use std::error::Error;
use std::io;
/*
   file: src/lib.rs
   author: jinshikai
*/

/*
   create date: 2/16/22
   desc: 运行函数主体
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    if config.route != "".to_string() {
        let key_vec = parse_key(config.route);
        loop {
            io::stdin().read_line(&mut input)?;
            if input.is_empty() {
                return Ok(());
            }
            let input_json = serde_json::from_str(&input).unwrap_or(Value::Null);
            let output = getter_from_vec(&input_json, &key_vec);
            match output {
                Value::Null => println!("{}", ""),
                other => println!(
                    "{}",
                    other
                        .to_string()
                        .trim_end_matches("\"")
                        .trim_start_matches("\"")
                ),
            }
            input.clear();
        }
    } else {
        Ok(())
    }
}

/*
   create date: 2/20/22
   desc: Value::Object的key的数据结构，可能为string或usize
*/
#[derive(Debug, PartialEq)]
pub enum JsonKey {
    Str(String),
    Idx(usize),
}

/*
   create date: 2/19/22
   desc: 从一个json中按vec的元素作为key依次取出value，中途遇到任何问题返回空；
   notice: 不区分缺失key/key存在但value为null这两种情况，一律返回Value::Null
*/
pub fn getter_from_vec<'a>(v: &'a Value, key_list: &Vec<JsonKey>) -> &'a Value {
    let mut ans = v;
    for key in key_list {
        match key {
            JsonKey::Str(key) => {
                let next = ans.get(key);
                match next {
                    None => return &Value::Null,
                    _ => {
                        ans = next.unwrap();
                        continue;
                    }
                }
            }
            JsonKey::Idx(key) => {
                let next = ans.get(key);
                match next {
                    None => return &Value::Null,
                    _ => {
                        ans = next.unwrap();
                        continue;
                    }
                }
            }
        }
    }
    return ans;
}

/*
   create date: 2/20/22
   desc: 解析一个字符串，返回一个vec:<JsonKey>
        1. 以'.'分隔每个item为一个vec
        2. 如果item以'['开头且以']'结尾且中间部分能转为数字，作为JsonKey::Idx;
        3. 其他情况作为JsonKey::Str;
*/
pub fn parse_key(raw_arg: String) -> Vec<JsonKey> {
    let v_raw: Vec<&str> = raw_arg.split(".").collect();
    let mut v_key: Vec<JsonKey> = Vec::new();
    for item in v_raw {
        let n = item.len();
        if n > 2 && &item[0..1] == "[" && &item[n - 1..n] == "]" {
            match &item[1..n - 1].parse() {
                Ok(num) => v_key.push(JsonKey::Idx(*num)),
                Err(_) => v_key.push(JsonKey::Str(item[1..n - 1].to_string())),
            };
        } else {
            v_key.push(JsonKey::Str(item.to_string()));
        }
    }
    return v_key;
}
/*
    create date:2/21/22
    desc: 命令行参数数据结构及构造函数
*/
pub struct Config {
    route: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let mut route = "".to_string();
        if args.len() > 1 {
            route = args[1].clone();
        }
        Config { route }
    }
}
