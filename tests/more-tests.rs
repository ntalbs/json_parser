use std::fs::read_to_string;
use std::error::Error;
use std::str::FromStr;

use json_parser::Json;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn parse_json_from_json_files() -> TestResult {
    let input = read_to_string("tests/inputs/list-lambda.json")?;

    match Json::from_str(input.as_str()) {
        Ok(json) => println!("{json}"),
        Err(_) => {
            panic!("failed to parse input json");
        }
    };

   Ok(())
}