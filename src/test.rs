/*
    author: jinshikai
    create date: 2/17/22
    desc: 测试驱动开发
*/
#[cfg(test)]
mod test {
    use serde_json::Value;
    // can be parsed as json, get json
    #[test]
    fn basic_ability_parse_json() {
        let contents = r#"{
            "name": "Shikai Jin",
            "age": "23"
        }"#;
        let v: Value = serde_json::from_str(contents).unwrap();
        assert_eq!("Shikai Jin", v["name"]);
    }

    // can't be parsed as json, get Null
    #[test]
    fn basic_ability_parse_general_str() {
        let contents = "abcd";
        let v: Value = serde_json::from_str(contents).unwrap_or(Value::Null);
        assert_eq!(Value::Null, v);
    }
}
