use std::collections::HashMap;
use std::string::String;

use crate::construct::{Error, Pos, Token, Json};
use crate::construct::Token::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    current: usize,
}

impl <'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Json, Error> {
        let json = self.json();
        match json {
            Json::Err(e) => Err(e),
            _ => Ok(json),
        }
    }

    fn json(&mut self) -> Json {
        let token = self.advance();
        match token {
            LeftBrace {..} => self.obj(),
            LeftBracket{..} => self.arr(),
            Token::String {val, ..} => {
                let s = val.to_string();
                self.str(s)
            },
            Number { val, .. } => {
                let n = *val;
                self.num(n)
            },
            Token::Bool { val, .. } => {
                let b = *val;
                self.boolean(b)
            },
            Token::Null {..} => self.null(),
            _ => {
                let pos = token.pos();
                self.err("Invalid Json".to_string(), pos)
            }
        }
    }

    fn obj(&mut self) -> Json {
        let mut m = HashMap::new();

        if matches!(self.peek(), RightBrace{..}) {
            self.advance();
            return Json::Obj(Box::new(m));
        }

        loop {
            let (key, val) = match self.member() {
                Ok((key, val)) => (key, val),
                Err(e) => return Json::Err(e)
            };
            m.insert(key, val);
            if !matches!(self.peek(), Comma{..}) {
                break;
            }
            self.advance();
        }

        if matches!(self.peek(), RightBrace{..}) {
            self.advance();
            Json::Obj(Box::new(m))
        } else {
            let pos = self.peek().pos();
            self.err("Invalid token: expected '}'".to_string(), pos)
        }
    }

    fn arr(&mut self) -> Json {
        if matches!(self.peek(), RightBracket{..}) {
            self.advance();
            return Json::Arr(vec![]);
        }

        let mut elements = Vec::new();
        elements.push(self.json());

        while matches!(self.peek(), Comma{..}) {
            self.advance();
            elements.push(self.json());
        }
        
        if matches!(self.peek(), RightBracket{..}) {
            self.advance();
            Json::Arr(elements)
        } else {
            let pos = self.peek().pos();
            self.err("Invalid token: expected ']'".to_string(), pos)
        }
    }

    fn member(&mut self) -> Result<(String, Json), Error> {
        let key = match &self.advance() {
            Token::String { val, ..} => val.to_string(),
            _ => return Err(Error {
                message: "Invalid token: expected string".to_string(),
                pos: self.peek().pos(),
            })
        };

        let val = match self.advance() {
            Colon{..} => self.json(),
            _ => return Err(Error {
                message: "Invalid token: expected ':'".to_string(),
                pos: self.peek().pos(),
            })
        };
        
        match val {
            Json::Err(e) => Err(e),
            _ => Ok((key, val))
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
        let e = Error {
            message,
            pos: pos,
        };
        Json::Err(e)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        *self.peek() == Token::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}