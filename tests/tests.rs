use std::str::FromStr;
use json_parser::Json;

#[test]
fn test_json_literals() {
    let json = Json::from_str("null").unwrap();
    assert_eq!(json, Json::Null);
    let json = Json::from_str("true").unwrap();
    assert_eq!(json, Json::Bool(true));
    let json = Json::from_str("false").unwrap();
    assert_eq!(json, Json::Bool(false));
    let json = Json::from_str("123.45").unwrap();
    assert_eq!(json, Json::Num(123.45));
    let json = Json::from_str("\"abc\"").unwrap();
    assert_eq!(json, Json::Str("abc".to_string()));
}

#[test]
fn test_json_empty() {
    let json = Json::from_str("{}").unwrap();
    assert_eq!(json, Json::Obj(vec![]));
    let json = Json::from_str("[]").unwrap();
    assert_eq!(json, Json::Arr(vec![]));
}

#[test]
fn test_json_obj() {
    let input = r#"{"a": 1, "b": 2}"#;
    let json = Json::from_str(input).unwrap();
    assert_eq!(
        json,
        Json::Obj(vec![
            ("a".to_string(), Json::Num(1.0)),
            ("b".to_string(), Json::Num(2.0)),
        ])
    );
}

#[test]
fn test_json_arr() {
    let input = r#"[1, 2, 3]"#;
    let json = Json::from_str(input).unwrap();
    assert_eq!(
        json,
        Json::Arr(vec![Json::Num(1.0), Json::Num(2.0), Json::Num(3.0),])
    );
}

#[test]
fn test_nested_empty_arr() {
    let input = r#"[[]]"#;
    let json = Json::from_str(input).unwrap();
    assert_eq!(json, Json::Arr(vec![Json::Arr(vec![])]));
}

#[test]
fn test_complex_obj() {
    let input = r#"
    {
        "a": 10,
        "b": true,
        "c": "hello",
        "d": [1, -2, 3.4, -5.6],
        "e": {
            "e1": true,
            "e2": false
        },
        "f": [
            {"a": 10},
            {"a": 20}
        ],
        "array_empty": [],
        "array_one": ["hello"],
        "object_empty": {},
        "object_one": {"hello": "world"}
    }"#;
    let json = Json::from_str(input).unwrap();
    assert_eq!(
        json,
        Json::Obj(vec![
            ("a".to_string(), Json::Num(10.0)),
            ("b".to_string(), Json::Bool(true)),
            ("c".to_string(), Json::Str("hello".to_string())),
            (
                "d".to_string(),
                Json::Arr(vec![
                    Json::Num(1.0),
                    Json::Num(-2.0),
                    Json::Num(3.4),
                    Json::Num(-5.6),
                ])
            ),
            (
                "e".to_string(),
                Json::Obj(vec![
                    ("e1".to_string(), Json::Bool(true)),
                    ("e2".to_string(), Json::Bool(false)),
                ])
            ),
            (
                "f".to_string(),
                Json::Arr(vec![
                    Json::Obj(vec![("a".to_string(), Json::Num(10.0))]),
                    Json::Obj(vec![("a".to_string(), Json::Num(20.0))]),
                ])
            ),
            ("array_empty".to_string(), Json::Arr(vec![])),
            (
                "array_one".to_string(),
                Json::Arr(vec![Json::Str("hello".to_string())])
            ),
            ("object_empty".to_string(), Json::Obj(vec![])),
            (
                "object_one".to_string(),
                Json::Obj(vec![("hello".to_string(), Json::Str("world".to_string()))])
            ),
        ])
    );
}
