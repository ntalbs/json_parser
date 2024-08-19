use json_parser::{Error, Json};
use std::io::{self, BufRead};
use std::str::FromStr;

fn print_errors(errors: Vec<Error>, input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    for l in &lines {
        println!("|{l}|");
    }

    for e in &errors {
        println!("{} at {}", e.message, e.pos);
        println!("{}", lines[e.pos.line - 1]);
        println!("{}^\n", " ".repeat(e.pos.col));
    }
}

fn main() {
    let mut src = String::new();
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        src.push_str(&line.unwrap());
    }

    match Json::from_str(&src) {
        Ok(json) => println!("{json}"),
        Err(errors) => {
            print_errors(errors, &src);
        }
    };
}
