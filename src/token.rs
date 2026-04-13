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
    Eof(Pos),
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
            Self::Eof(pos) => pos,
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

impl Display for Token<'_> {
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
            Self::Eof(_) => f.write_str("EOF"),
        }
    }
}

#[cfg(test)]
mod test {
    use p_test::p_test;

    use crate::{Pos, token::Token};

    const POS: Pos = Pos { line: 1, col: 1 };

    #[p_test(
        (Token::LeftBrace(POS)),
        (Token::RightBrace(POS)),
        (Token::LeftBracket(POS)),
        (Token::RightBracket(POS)),
        (Token::Colon(POS)),
        (Token::Comma(POS)),
        (Token::String { lexeme: "dummy", val: "dummy", pos: POS} ),
        (Token::Number { lexeme: "100", val: 100.0, pos: POS} ),
        (Token::Bool { lexeme: "true", val: true, pos: POS} ),
        (Token::Null { lexeme: "null", pos: POS} ),
        (Token::Eof(POS)),
    )]
    fn test_token_pos(token: Token) {
        assert_eq!(token.pos(), POS);
    }

    #[p_test(
        (Token::LeftBrace(POS), "{"),
        (Token::RightBrace(POS), "}"),
        (Token::LeftBracket(POS), "["),
        (Token::RightBracket(POS), "]"),
        (Token::Colon(POS), ":"),
        (Token::Comma(POS), ","),
        (Token::String { lexeme: "dummy", val: "dummy", pos: POS}, "dummy" ),
        (Token::Number { lexeme: "100", val: 100.0, pos: POS}, "100" ),
        (Token::Bool { lexeme: "true", val: true, pos: POS}, "true" ),
        (Token::Null { lexeme: "null", pos: POS}, "null" ),
    )]
    fn test_token_lexeme(token: Token, lexeme: &str) {
        assert_eq!(token.lexeme(), lexeme);
    }

    #[test]
    #[should_panic]
    fn test_token_lexeme_eof() {
        Token::Eof(POS).lexeme();
    }

    #[p_test(
        (Token::LeftBrace(POS), "'{' at 1:1"),
        (Token::RightBrace(POS), "'}' at 1:1"),
        (Token::LeftBracket(POS), "'[' at 1:1"),
        (Token::RightBracket(POS), "']' at 1:1"),
        (Token::Colon(POS), "':' at 1:1"),
        (Token::Comma(POS), "',' at 1:1"),
        (Token::String { lexeme: "dummy", val: "dummy", pos: POS}, "'dummy' => \"dummy\" at 1:1"),
        (Token::Number { lexeme: "100", val: 100.0, pos: POS}, "'100' => 100 at 1:1"),
        (Token::Bool { lexeme: "true", val: true, pos: POS}, "'true' => true at 1:1"),
        (Token::Null { lexeme: "null", pos: POS}, "'null' => null at 1:1"),
        (Token::Eof(POS), "EOF"),
    )]
    fn test_token_to_string(token: Token, expected: &str) {
        assert_eq!(token.to_string(), expected);
    }
}
