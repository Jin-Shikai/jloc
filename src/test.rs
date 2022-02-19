/*
    author: jinshikai
    create date: 2/17/22
    desc: 测试驱动开发
*/
#[cfg(test)]
mod test {
    use serde_json::{json, Value};
    use std::io;

    // if str can be parsed as json, get json
    #[test]
    fn parse_json() {
        let contents = r#"{
            "name": "Shikai Jin",
            "age": "23",
            "age_int":23
        }"#;
        let v: Value = serde_json::from_str(contents).unwrap();
        assert_eq!("Shikai Jin", v["name"]);
        assert_eq!("23", v["age"]);
        assert_eq!(23, v["age_int"]);
    }

    // elif can't be parsed as json, get Null
    #[test]
    fn parse_general_str() {
        let contents = "abcd";
        let v: Value = serde_json::from_str(contents).unwrap_or(Value::Null);
        assert_eq!(json!(Value::Null), v);
    }

    #[test]
    fn parse_general_int() {
        assert_eq!(json!("1234"), "1234");
        let contents = "1234";
        let v: Value = serde_json::from_str(contents).unwrap_or(Value::Null);
        assert_eq!(json!(1234), v);
    }
}
