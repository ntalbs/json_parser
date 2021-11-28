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

    let mut scanner = Scanner::new(input);

    fn print_error(e: Error) {
        println!("{} at {}", e.message, e.pos);
    }

    fn print_errors(errors: &Vec<Error>) {
        for e in errors {
            println!("{} at {}", e.message, e.pos);
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
            print_errors(errors);
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
