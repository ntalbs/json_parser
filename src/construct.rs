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
    LeftBrace(Pos),
    RightBrace(Pos),
    LeftBracket(Pos),
    RightBracket(Pos),
    Colon(Pos),
    Comma(Pos),
    String(&'a str, Pos),
    Number(f64, Pos),
    Bool(bool, Pos),
    Null(Pos),
    EOF,
}

impl <'a> Token<'a> {
    pub fn pos(&self) -> Pos {
        match self {
            Self::LeftBrace(pos) => *pos,
            Self::RightBrace(pos) => *pos,
            Self::LeftBracket(pos) => *pos,
            Self::RightBracket(pos) => *pos,
            Self::Colon(pos) => *pos,
            Self::Comma(pos) => *pos,
            Self::String(_, pos) => *pos,
            Self::Number(_, pos) => *pos,
            Self::Bool(_, pos) => *pos,
            Self::Null(pos) => *pos,
            _ => panic!("EOF doesn't have pos!"),
        }
    }
}

impl <'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftBrace(pos) =>
                f.write_fmt(format_args!("'{{' at {}", pos)),
            Self::RightBrace(pos) =>
                f.write_fmt(format_args!("'}}' at {}", pos)),
            Self::LeftBracket(pos) =>
                f.write_fmt(format_args!("'[' at {}", pos)),
            Self::RightBracket(pos) =>
                f.write_fmt(format_args!("']' at {}", pos)),
            Self::Colon(pos) =>
                f.write_fmt(format_args!("':' at {}", pos)),
            Self::Comma(pos) =>
                f.write_fmt(format_args!("'{{' at {}", pos)),
            Self::String(s, pos) =>
                f.write_fmt(format_args!("\"{}\" at {}", s, pos)),
            Self::Number(n, pos) =>
                f.write_fmt(format_args!("{} at {}", n, pos)),
            Self::Bool(b, pos) =>
                f.write_fmt(format_args!("{} at {}", b, pos)),
            Self::Null(pos) =>
                f.write_fmt(format_args!("null at {}", pos)),
            Self::EOF => f.write_str("EOF"),
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
