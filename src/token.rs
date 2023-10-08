use std::fmt::Display;

use crate::Pos;

#[derive(Debug, PartialEq)]
pub(crate) enum Token<'a> {
    LeftBrace(Pos),
    RightBrace(Pos),
    LeftBracket(Pos),
    RightBracket(Pos),
    Colon(Pos),
    Comma(Pos),
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
    pub(crate) fn pos(&self) -> Pos {
        match *self {
            Self::LeftBrace(pos) => pos,
            Self::RightBrace(pos) => pos,
            Self::LeftBracket(pos) => pos,
            Self::RightBracket(pos) => pos,
            Self::Colon(pos) => pos,
            Self::Comma(pos) => pos,
            Self::String { pos, .. } => pos,
            Self::Number { pos, .. } => pos,
            Self::Bool { pos, .. } => pos,
            Self::Null { pos, .. } => pos,
            _ => panic!("EOF doesn't have pos!"),
        }
    }

    pub(crate) fn lexeme(&self) -> &'a str {
        match *self {
            Self::LeftBrace { .. } => "{",
            Self::RightBrace { .. } => "}",
            Self::LeftBracket { .. } => "[",
            Self::RightBracket { .. } => "]",
            Self::Colon { .. } => ":",
            Self::Comma { .. } => ",",
            Self::String { lexeme, .. } => lexeme,
            Self::Number { lexeme, .. } => lexeme,
            Self::Bool { lexeme, .. } => lexeme,
            Self::Null { lexeme, .. } => lexeme,
            _ => panic!("EOF doesn't have pos!"),
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftBrace { .. }
            | Self::RightBrace { .. }
            | Self::LeftBracket { .. }
            | Self::RightBracket { .. }
            | Self::Colon { .. }
            | Self::Comma { .. } => {
                f.write_fmt(format_args!("'{}' at {}", self.lexeme(), self.pos()))
            }
            Self::String { lexeme, pos, val } => {
                f.write_fmt(format_args!("'{lexeme}' => \"{val}\" at {pos}"))
            }
            Self::Number { lexeme, pos, val } => {
                f.write_fmt(format_args!("'{lexeme}' => {val} at {pos}"))
            }
            Self::Bool { lexeme, pos, val } => {
                f.write_fmt(format_args!("'{lexeme}' => {val} at {pos}"))
            }
            Self::Null { lexeme, pos } => f.write_fmt(format_args!("'{lexeme}' => null at {pos}")),
            Self::Eof => f.write_str("EOF"),
        }
    }
}
