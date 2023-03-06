use std::fmt::Display;

use crate::Pos;

#[derive(Debug, PartialEq)]
pub(crate) enum Token<'a> {
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
    pub(crate) fn pos(&self) -> Pos {
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
            | Self::Comma { lexeme, pos } => f.write_fmt(format_args!("'{lexeme}' at {pos}")),
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
