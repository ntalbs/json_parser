mod color;
mod parser;
mod scanner;
mod token;

use crate::color::Color;
use std::{collections::HashMap, fmt::Display, str::FromStr};

use parser::Parser;
use scanner::Scanner;
use token::Token;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.line, self.col))
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub pos: Pos,
}

#[derive(Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Obj(Vec<(String, Json)>),
    Arr(Vec<Json>),
}

impl FromStr for Json {
    type Err = Vec<Error>;

    fn from_str(s: &str) -> Result<Self, Vec<Error>> {
        let mut line_map: HashMap<usize, &str> = HashMap::new();
        for line in s.lines().enumerate() {
            line_map.insert(line.0 + 1, line.1);
        }

        let mut scanner = Scanner::new(s);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => {
                println!(">>> Tokens:");
                for t in tokens {
                    match t {
                        Token::Eof => break,
                        _ => {}
                    }
                    let line = line_map.get(&t.pos().line).unwrap();
                    let col = t.pos().col;
                    println!("{t}");
                    println!("{:>6} | {}", t.pos().line, line);
                    println!("        {}^", " ".repeat(col));
                }
                println!(">>>");
                tokens
            }
            Err(errors) => return Err(errors.to_vec()),
        };

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(json) => Ok(json),
            Err(error) => Err(vec![error]),
        }
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TAB: &str = "  ";
        fn indent_first(level: usize, is_under_key: bool) -> String {
            TAB.repeat(if is_under_key { 0 } else { level })
        }

        fn indent_body(level: usize) -> String {
            TAB.repeat(level + 1)
        }

        fn indent_last(level: usize) -> String {
            TAB.repeat(level)
        }

        fn fmt_object(
            f: &mut std::fmt::Formatter<'_>,
            obj: &[(String, Json)],
            level: usize,
            is_under_key: bool,
        ) -> std::fmt::Result {
            let indent_first = indent_first(level, is_under_key);
            let indent_body = indent_body(level);
            let indent_last = indent_last(level);

            match obj {
                [] => f.write_fmt(format_args!("{indent_first}{{}}")),
                [h, tail @ ..] => {
                    f.write_fmt(format_args!("{indent_first}{{\n"))?;
                    let (k, v) = h;
                    f.write_fmt(format_args!("{indent_body}\"{}\": ", k.bright_green()))?;
                    fmt_level(f, v, level + 1, true)?;
                    for (k, v) in tail {
                        f.write_fmt(format_args!(",\n{indent_body}\"{}\": ", k.bright_green()))?;
                        fmt_level(f, v, level + 1, true)?;
                    }
                    f.write_fmt(format_args!("\n{indent_last}}}"))
                }
            }
        }

        fn fmt_array(
            f: &mut std::fmt::Formatter<'_>,
            arr: &[Json],
            level: usize,
            is_under_key: bool,
        ) -> std::fmt::Result {
            let indent_first = indent_first(level, is_under_key);
            let indent_last = indent_last(level);

            match arr {
                [] => f.write_fmt(format_args!("{indent_first}[]")),
                [h, tail @ ..] => {
                    f.write_fmt(format_args!("{indent_first}[\n"))?;
                    fmt_level(f, h, level + 1, false)?;
                    for e in tail {
                        f.write_str(",\n")?;
                        fmt_level(f, e, level + 1, false)?;
                    }
                    f.write_fmt(format_args!("\n{indent_last}]"))
                }
            }
        }

        fn fmt_level(
            f: &mut std::fmt::Formatter<'_>,
            json: &Json,
            level: usize,
            is_under_key: bool,
        ) -> std::fmt::Result {
            let indent_first = indent_first(level, is_under_key);

            match json {
                Json::Null => f.write_fmt(format_args!("{indent_first}{}", "null".bright_yellow())),
                Json::Bool(v) => f.write_fmt(format_args!(
                    "{indent_first}{}",
                    v.to_string().bright_cyan()
                )),
                Json::Num(v) => f.write_fmt(format_args!(
                    "{indent_first}{}",
                    v.to_string().bright_magenta()
                )),
                Json::Str(v) => f.write_fmt(format_args!("{indent_first}\"{}\"", v.bright_blue())),
                Json::Obj(m) => fmt_object(f, m, level, is_under_key),
                Json::Arr(arr) => fmt_array(f, arr, level, is_under_key),
            }
        }

        fmt_level(f, self, 0, false)
    }
}
