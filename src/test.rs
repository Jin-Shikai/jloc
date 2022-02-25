/*
    author: jinshikai
    create date: 2/17/22
    desc: 测试驱动开发
*/

#[cfg(test)]
mod test {
    use serde_json::*;
    use std::*;

    use crate::*;

    #[test]
    fn test_parse_json() {
        let contents: Value = json!(
            {
                "name": "Shikai Jin",
                "age": "23",
                "age_int":23
            }
        );
        assert_eq!("Shikai Jin", contents["name"]);
        assert_eq!("23", contents["age"]);
        assert_eq!(23, contents["age_int"]);
    }

    #[test]
    fn test_parse_general_str() {
        let contents = "abcd";
        let v: Value = serde_json::from_str(contents).unwrap_or(Value::Null);
        assert_eq!(json!(Value::Null), v);
    }

    #[test]
    fn test_parse_general_int() {
        let contents = "1234";
        let v: Value = serde_json::from_str(contents).unwrap_or(Value::Null);
        assert_eq!(json!(1234), v);
        assert_eq!(json!("1234"), "1234");
    }

    #[test]
    fn test_format_output() {
        let input = json!(
            {
                "a boolean": false,
                "an array": [3, 2, 1],
                "child_json":{"test_key" :"test_value"}
            }
        );
        let mut serializer = serde_json::Serializer::pretty(io::stdout());
        serde_transcode::transcode(input, &mut serializer).unwrap();
    }

    #[test]
    fn test_getter() {
        let input = json!(
            {
                "a boolean": false,
                "an array": [3, 2, 1],
                "child_json":{"test_key" :"test_value"}
            }
        );
        assert_eq!(json!(false), *input.get("a boolean").unwrap());
        assert_eq!(json!([3, 2, 1]), *input.get("an array").unwrap());
        assert_eq!(json!(3), *input.get("an array").unwrap().get(0).unwrap());
        assert_eq!(None, input.get(0));
        assert_eq!(None, input.get("not exisit"));
    }

    #[test]
    fn test_getter_from_vec() {
        let input = json!(
            {
                "a boolean": null,
                "an array": [3, 2, 1],
                "child_json":{"test_key" :"test_value"}
            }
        );
        let key_list_corr_1: Vec<JsonKey> =
            vec![JsonKey::Str("an array".to_string()), JsonKey::Idx(0)];
        assert_eq!(json!(3), *getter_from_vec(&input, &key_list_corr_1));

        let key_list_corr_2: Vec<JsonKey> = vec![JsonKey::Str("a boolean".to_string())];
        assert_eq!(Value::Null, *getter_from_vec(&input, &key_list_corr_2));

        let key_list_corr_3: Vec<JsonKey> = vec![
            JsonKey::Str("child_json".to_string()),
            JsonKey::Str("test_key".to_string()),
        ];
        assert_eq!(
            json!("test_value"),
            *getter_from_vec(&input, &key_list_corr_3)
        );
        let key_list_err: Vec<JsonKey> =
            vec![JsonKey::Str("not exisit".to_string()), JsonKey::Idx(0)];
        assert_eq!(Value::Null, *getter_from_vec(&input, &key_list_err));
    }

    #[test]
    fn test_parse_key() {
        let arg_str = "a.b.[1].c.d";
        let target_vec: Vec<JsonKey> = vec![
            JsonKey::Str("a".to_string()),
            JsonKey::Str("b".to_string()),
            JsonKey::Idx(1),
            JsonKey::Str("c".to_string()),
            JsonKey::Str("d".to_string()),
        ];
        assert_eq!(target_vec, parse_key(arg_str));
    }

    #[test]
    fn test_parse_and_get() {
        let input = json!(
            {
                "key1": "a",
                "key2": [3, 2, 1],
                "key3": {
                    "test_key": "test_value"
                },
                "key4": [{
                    "k4_1": "v4_1"
                }, {
                    "k4_2": "v4_2"
                }],
                "key5": [
                    "a",
                    "b",
                    "c"
                ]
            }
        );
        let mut arg_str = "key3.test_key";
        assert_eq!(
            json!("test_value"),
            *getter_from_vec(&input, &parse_key(arg_str))
        );
        arg_str = "key1";
        assert_eq!(json!("a"), *getter_from_vec(&input, &parse_key(arg_str)));
        arg_str = "key2.[0]";
        assert_eq!(json!(3), *getter_from_vec(&input, &parse_key(arg_str)));
        arg_str = "key4.[0].k4_1";
        assert_eq!(json!("v4_1"), *getter_from_vec(&input, &parse_key(arg_str)));
    }

    #[test]
    fn test_split() {
        let input = "abc=zxc";
        let input: Vec<&str> = input.split("=").collect();
        assert_eq!(input[0], "abc");
        assert_eq!(input[1], "zxc");
        let input = "abc";
        let input: Vec<&str> = input.split("=").collect();
        assert_eq!(input[0], "abc");
    }
}
