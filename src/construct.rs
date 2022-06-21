use std::collections::BTreeMap;
use std::fmt::Display;

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

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    LeftBrace {
        lexeme: &'a str,
        pos: Pos,
    },
    RightBrace {
        lexeme: &'a str,
        pos: Pos,
    },
    LeftBracket {
        lexeme: &'a str,
        pos: Pos,
    },
    RightBracket {
        lexeme: &'a str,
        pos: Pos,
    },
    Colon {
        lexeme: &'a str,
        pos: Pos,
    },
    Comma {
        lexeme: &'a str,
        pos: Pos,
    },
    String {
        lexeme: &'a str,
        val: &'a str,
        pos: Pos,
    },
    Number {
        lexeme: &'a str,
        val: f64,
        pos: Pos,
    },
    Bool {
        lexeme: &'a str,
        val: bool,
        pos: Pos,
    },
    Null {
        lexeme: &'a str,
        pos: Pos,
    },
    Eof,
}

impl<'a> Token<'a> {
    pub fn pos(&self) -> Pos {
        match self {
            Self::LeftBrace { pos, .. } => *pos,
            Self::RightBrace { pos, .. } => *pos,
            Self::LeftBracket { pos, .. } => *pos,
            Self::RightBracket { pos, .. } => *pos,
            Self::Colon { pos, .. } => *pos,
            Self::Comma { pos, .. } => *pos,
            Self::String { pos, .. } => *pos,
            Self::Number { pos, .. } => *pos,
            Self::Bool { pos, .. } => *pos,
            Self::Null { pos, .. } => *pos,
            _ => panic!("EOF doesn't have pos!"),
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftBrace { lexeme, pos }
            | Self::RightBrace { lexeme, pos }
            | Self::LeftBracket { lexeme, pos }
            | Self::RightBracket { lexeme, pos }
            | Self::Colon { lexeme, pos }
            | Self::Comma { lexeme, pos } => f.write_fmt(format_args!("'{}' at {}", lexeme, pos)),
            Self::String { lexeme, pos, val } => {
                f.write_fmt(format_args!("'{}' => \"{}\" at {}", lexeme, val, pos))
            }
            Self::Number { lexeme, pos, val } => {
                f.write_fmt(format_args!("'{}' => {} at {}", lexeme, val, pos))
            }
            Self::Bool { lexeme, pos, val } => {
                f.write_fmt(format_args!("'{}' => {} at {}", lexeme, val, pos))
            }
            Self::Null { lexeme, pos } => {
                f.write_fmt(format_args!("'{}' => null at {}", lexeme, pos))
            }
            Self::Eof => f.write_str("EOF"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Obj(BTreeMap<String, Json>),
    Arr(Vec<Json>),
    Err(Error),
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
            obj: &BTreeMap<String, Json>,
            level: usize,
            is_under_key: bool,
        ) -> std::fmt::Result {
            let indent_first = indent_first(level, is_under_key);
            let indent_body = indent_body(level);
            let indent_last = indent_last(level);

            let mut is_first = true;
            f.write_fmt(format_args!("{indent_first}{{\n"))?;
            for (k, v) in obj {
                if !is_first {
                    f.write_str(",\n")?;
                }
                f.write_fmt(format_args!("{indent_body}\"{k}\": "))?;
                fmt_level(f, v, level + 1, true)?;
                is_first = false;
            }
            f.write_fmt(format_args!("\n{indent_last}}}"))
        }

        fn fmt_array(
            f: &mut std::fmt::Formatter<'_>,
            arr: &[Json],
            level: usize,
            is_under_key: bool,
        ) -> std::fmt::Result {
            let indent_first = indent_first(level, is_under_key);
            let indent_last = indent_last(level);

            let mut is_first = true;
            f.write_fmt(format_args!("{indent_first}[\n"))?;
            for e in arr {
                if !is_first {
                    f.write_str(",\n")?;
                }
                fmt_level(f, e, level + 1, false)?;
                is_first = false;
            }
            f.write_fmt(format_args!("\n{indent_last}]"))
        }

        fn fmt_level(
            f: &mut std::fmt::Formatter<'_>,
            json: &Json,
            level: usize,
            is_under_key: bool,
        ) -> std::fmt::Result {
            let indent_first = indent_first(level, is_under_key);

            match json {
                Json::Null => f.write_fmt(format_args!("{indent_first}null")),
                Json::Bool(v) => f.write_fmt(format_args!("{indent_first}{}", v)),
                Json::Num(v) => f.write_fmt(format_args!("{indent_first}{}", v)),
                Json::Str(v) => f.write_fmt(format_args!("{indent_first}\"{}\"", v)),
                Json::Obj(m) => fmt_object(f, m, level, is_under_key),
                Json::Arr(arr) => fmt_array(f, arr, level, is_under_key),
                Json::Err(e) => f.write_fmt(format_args!("Error: {} at {}", e.message, e.pos)),
            }
        }

        fmt_level(f, self, 0, false)
    }
}
