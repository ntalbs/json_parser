use std::{collections::HashMap, fmt::Display};

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
    LeftBrace    { lexeme: &'a str, pos: Pos },
    RightBrace   { lexeme: &'a str, pos: Pos },
    LeftBracket  { lexeme: &'a str, pos: Pos },
    RightBracket { lexeme: &'a str, pos: Pos },
    Colon        { lexeme: &'a str, pos: Pos },
    Comma        { lexeme: &'a str, pos: Pos },
    String       { lexeme: &'a str, val: &'a str, pos: Pos },
    Number       { lexeme: &'a str, val: f64, pos: Pos },
    Bool         { lexeme: &'a str, val: bool, pos: Pos },
    Null         { lexeme: &'a str, pos: Pos },
    EOF,
}

impl <'a> Token<'a> {
    pub fn pos(&self) -> Pos {
        match self {
            Self::LeftBrace    { pos, .. } => *pos,
            Self::RightBrace   { pos, .. } => *pos,
            Self::LeftBracket  { pos, .. } => *pos,
            Self::RightBracket { pos, .. } => *pos,
            Self::Colon        { pos, .. } => *pos,
            Self::Comma        { pos, .. } => *pos,
            Self::String       { pos, .. } => *pos,
            Self::Number       { pos, .. } => *pos,
            Self::Bool         { pos, .. } => *pos,
            Self::Null         { pos, .. } => *pos,
            _ => panic!("EOF doesn't have pos!"),
        }
    }
}

impl <'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftBrace    { lexeme, pos, .. } |
            Self::RightBrace   { lexeme, pos, .. } |
            Self::LeftBracket  { lexeme, pos, .. } |
            Self::RightBracket { lexeme, pos, .. } |
            Self::Colon        { lexeme, pos, .. } |
            Self::Comma        { lexeme, pos, .. } |
            Self::String       { lexeme, pos, .. } |
            Self::Number       { lexeme, pos, .. } |
            Self::Bool         { lexeme, pos, .. } |
            Self::Null         { lexeme, pos, .. } =>
                f.write_fmt(format_args!("'{}' at {}", lexeme, pos)),
            Self::EOF =>
                f.write_str("EOF"),
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
    Obj(Box<HashMap<String, Json>>),
    Arr(Vec<Json>),
    Err(Error),
}
