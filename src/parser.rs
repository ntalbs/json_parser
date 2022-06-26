use std::string::String;

use crate::construct::Token::*;
use crate::construct::{Error, Json, Pos, Token};

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    errors: Vec<Error>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { 
            tokens,
            errors: Vec::new(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Json, Vec<Error>> {
        let json = self.json();

        if self.errors.is_empty() {
            Ok(json)
        } else {
            Err(self.errors)
        }
    }

    fn json(&mut self) -> Json {
        let token = self.advance();
        match token {
            LeftBrace { .. } => self.obj(),
            LeftBracket { .. } => self.arr(),
            Token::String { val, .. } => {
                let s = val.to_string();
                self.str(s)
            }
            Number { val, .. } => {
                let n = *val;
                self.num(n)
            }
            Token::Bool { val, .. } => {
                let b = *val;
                self.boolean(b)
            }
            Token::Null { .. } => self.null(),
            _ => {
                let pos = token.pos();
                self.err("Invalid Json".to_string(), pos)
            }
        }
    }

    fn obj(&mut self) -> Json {
        let mut m: Vec<(String, Json)> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();

        if matches!(self.peek(), RightBrace { .. }) {
            self.advance();
            return Json::Obj(m);
        }

        loop {
            match self.member() {
                Ok((key, val)) => m.push((key, val)),
                Err(errs) => for e in errs {
                    errors.push(e);
                }
            };
            if !matches!(self.peek(), Comma { .. }) {
                break;
            }
            self.advance();
        }

        if matches!(self.peek(), RightBrace { .. }) {
            self.advance();
            Json::Obj(m)
        } else {
            let pos = self.peek().pos();
            errors.push(Error {
                message: "Invalid token: expected '}'".to_string(),
                pos
            });
            Json::Err(errors)
        }
    }

    fn arr(&mut self) -> Json {
        if matches!(self.peek(), RightBracket { .. }) {
            self.advance();
            return Json::Arr(vec![]);
        }

        let mut elements = vec![self.json()];
        //let mut errors: Vec<Error> = Vec::new();

        while matches!(self.peek(), Comma { .. }) {
            self.advance();
            elements.push(self.json());
        }

        if matches!(self.peek(), RightBracket { .. }) {
            self.advance();
            Json::Arr(elements)
        } else {
            let pos = self.peek().pos();
            self.err("Invalid token: expected ']'".to_string(), pos)
        }
    }

    fn member(&mut self) -> Result<(String, Json), Vec<Error>> {
        let key = match &self.advance() {
            Token::String { val, .. } => val.to_string(),
            _ => {
                return Err(vec![Error {
                    message: "Invalid token: expected string".to_string(),
                    pos: self.peek().pos(),
                }])
            }
        };

        let val = match self.advance() {
            Colon { .. } => self.json(),
            _ => {
                return Err(vec![Error {
                    message: "Invalid token: expected ':'".to_string(),
                    pos: self.peek().pos(),
                }])
            }
        };

        match val {
            Json::Err(errors) => Err(errors),
            _ => Ok((key, val)),
        }
    }

    fn str(&mut self, s: String) -> Json {
        Json::Str(s)
    }

    fn num(&mut self, n: f64) -> Json {
        Json::Num(n)
    }

    fn boolean(&mut self, b: bool) -> Json {
        Json::Bool(b)
    }

    fn null(&mut self) -> Json {
        Json::Null
    }

    fn err(&mut self, message: String, pos: Pos) -> Json {
        let e = Error { message, pos };
        Json::Err(vec![e])
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        *self.peek() == Token::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
