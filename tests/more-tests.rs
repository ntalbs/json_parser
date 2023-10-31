use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

use json_parser::Json;

type TestResult = Result<(), Box<dyn Error>>;

macro_rules! json_tests {
    ( $(($test_name:ident, $file_name:literal) $(,)?)* ) => {$(
        #[test]
        fn $test_name() -> TestResult {
            parse_json($file_name)?;
            Ok(())
        }
    )*}
}

fn parse_json(file_name: &str) -> TestResult {
    let path = format!("tests/inputs/{}", file_name);
    let input = read_to_string(path)?;
    if let Ok(_) = Json::from_str(input.as_str()) {
        return Ok(());
    } else {
        panic!();
    }
}

json_tests!(
    (list_lambda, "list-lambda.json"),
    (manifest, "manifest.json"),
    (tree, "tree.json"),
    (tweetbook_stack_assets, "TweetbookStack.assets.json"),
    (tweetbook_stack_template, "TweetbookStack.template.json"),
);
