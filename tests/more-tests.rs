use std::error::Error;
use std::fs::{self, read_to_string};
use std::io::Write;
use std::str::FromStr;

use json_parser::Json;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn parse_json_from_json_files() -> TestResult {
    let paths = fs::read_dir("tests/inputs")?;
    for f in paths {
        let dir_entry = f?;
        let path = dir_entry.path();
        let file = path.to_str().unwrap();
        let input = read_to_string(file)?;
        println!(">>> {file}");
        match Json::from_str(input.as_str()) {
            Ok(json) => println!("{json}"),
            Err(e) => {
                println!(">>> Failed to parse {file}");
                println!("{e:?}");
                std::io::stdout().flush()?;
            }
        }
    }

    Ok(())
}
