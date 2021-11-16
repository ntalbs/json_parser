mod scanner;
mod parser;

use scanner::Scanner;
use parser::Parser;

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

    match scanner.scan_tokens() {
        Ok(tokens) => {
            for t in tokens {
                println!("{:?}", t);
            }
        }
        Err(errors) => {
            for e in errors {
                println!("{:?}", e);
            }
            std::process::exit(1);
        }
    }

    let mut parser = Parser::new(scanner.tokens);
    let json = parser.parse();

    println!("{:?}", json);
}
