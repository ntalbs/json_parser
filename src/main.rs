mod scanner;

use scanner::Scanner;

fn main() {
    let input = r#"
    {
        "a": 10,
        "b": true,
        "c": "hello",
        "d": [1, 2, 3],
        "e": {
            "e1": true,
            "e2": false
        },
        "f": [
            {"a": 10},
            {"a": 20}
        ]
    }
    "#;

    let mut scanner = Scanner::new(input.to_string());

    for t in scanner.scan_tokens() {
        println!("{:?}", t);
    }
}
