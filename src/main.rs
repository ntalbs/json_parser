mod scanner;
mod parser;
mod construct;

use scanner::Scanner;
use parser::Parser;
use construct::Error;

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
        ]
    }
    "#;

    let mut scanner = Scanner::new(input);

    fn print_error(e: Error) {
        println!("{} at {}", e.message, e.pos);
    }

    fn print_errors(errors: &Vec<Error>, input: &str) {
        let lines: Vec<&str> = input.lines().collect();
        for l in &lines {
            println!("|{}|", l);
        }

        for e in errors {
            println!("{} at {}", e.message, e.pos);
            println!("{}", lines[e.pos.line-1]);
            println!("{}^\n", " ".repeat(e.pos.col));
        }
    }

    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => {
            for t in tokens {
                println!("{}", t);
            }
            tokens
        }
        Err(errors) => {
            print_errors(errors, input);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let json = match  parser.parse() {
        Ok(json) =>  json,
        Err(e) => {
            print_error(e);
            std::process::exit(1);
        }
    };

    println!("{:?}", json);
}
