use std::string::String;

use crate::construct::Token::*;
use crate::construct::{Error, Json, Token};

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { 
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Json, Error> {
        self.json()
    }

    fn json(&mut self) -> Result<Json, Error> {
        let token = self.advance();
        match token {
            LeftBrace { .. } => self.obj(),
            LeftBracket { .. } => self.arr(),
            Token::String { val, .. } => {
                let s = val.to_string();
                Ok(self.str(s))
            }
            Number { val, .. } => {
                let n = *val;
                Ok(self.num(n))
            }
            Token::Bool { val, .. } => {
                let b = *val;
                Ok(self.boolean(b))
            }
            Token::Null { .. } => Ok(self.null()),
            _ => {
                let pos = token.pos();
                Err(Error {
                    message: "Invalid Json".to_string(),
                    pos
                })
            }
        }
    }

    fn obj(&mut self) -> Result<Json, Error> {
        let mut m: Vec<(String, Json)> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();

        if matches!(self.peek(), RightBrace { .. }) {
            self.advance();
            return Ok(Json::Obj(m));
        }

        loop {
            match self.member() {
                Ok((key, val)) => m.push((key, val)),
                Err(e) => errors.push(e),
            };
            if !matches!(self.peek(), Comma { .. }) {
                break;
            }
            self.advance();
        }

        if matches!(self.peek(), RightBrace { .. }) {
            self.advance();
            Ok(Json::Obj(m))
        } else {
            let pos = self.peek().pos();
            // errors???
            Err(Error {
                message: "Invalid token: expected '}'".to_string(),
                pos
            })
        }
    }

    fn arr(&mut self) -> Result<Json, Error> {
        let mut elements:Vec<Json> = Vec::new();
        if matches!(self.peek(), RightBracket { .. }) {
            self.advance();
            return Ok(Json::Arr(elements));
        }

        loop {
            match self.json() {
                Ok(e) => elements.push(e),
                Err(e) => return Err(e),
            }

            if !matches!(self.peek(), Comma { .. }) {
                break;
            }
            self.advance();
        }

        if matches!(self.peek(), RightBracket { .. }) {
            self.advance();
            Ok(Json::Arr(elements))
        } else {
            let pos = self.peek().pos();
            Err(Error {
                message: "Invalid token: expected ']'".to_string(),
                pos
            })
        }
    }

    fn member(&mut self) -> Result<(String, Json), Error> {
        let key = match &self.advance() {
            Token::String { val, .. } => val.to_string(),
            _ => {
                return Err(Error {
                    message: "Invalid token: expected string".to_string(),
                    pos: self.peek().pos(),
                });
            }
        };

        let val = match self.advance() {
            Colon { .. } => match self.json() {
                Ok(val) => val,
                Err(e) => return Err(e),
            },
            _ => {
                return Err(Error {
                    message: "Invalid token: expected ':'".to_string(),
                    pos: self.peek().pos(),
                })
            }
        };

        Ok((key, val))
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

    // fn err(&mut self, message: String, pos: Pos) -> Json {
    //     let e = Error { message, pos };
    //     Json::Err(vec![e])
    // }

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
