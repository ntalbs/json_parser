mod construct;
mod parser;
mod scanner;

use crate::construct::{Json, Error};

fn main() {
    let input = r#"
    {
        "a": 10,
        "b": true,
        "c": "hello",
        "d": [1, 2, 3.1, -4.2],
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
    }
    "#;

    fn print_errors(errors: Vec<Error>, input: &str) {
        let lines: Vec<&str> = input.lines().collect();
        for l in &lines {
            println!("|{}|", l);
        }

        for e in &errors {
            println!("{} at {}", e.message, e.pos);
            println!("{}", lines[e.pos.line - 1]);
            println!("{}^\n", " ".repeat(e.pos.col));
        }
    }

    match Json::from_str(input) {
        Ok(json) => println!("{}", json),
        Err(errors) => {
            print_errors(errors, input);
        }
    };
}
