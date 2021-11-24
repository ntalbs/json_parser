use std::collections::HashMap;
use std::string::String;

use crate::scanner::{Error, Token};
use crate::scanner::Token::*;

#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Obj(Box<HashMap<String, Json>>),
    Arr(Vec<Json>),
}

pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<Error>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            errors: Vec::new(),
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Json, &Vec<Error>> {
        let json = self.json();
        if self.errors.is_empty() {
            Ok(json)
        } else {
            Err(&self.errors)
        }
    }

    fn json(&mut self) -> Json {
        let token = self.peek();
        match token {
            LeftBrace(_) => self.obj(),
            LeftBracket(_) => self.arr(),
            String(s, _) => {
                let s = s.to_string();
                self.str(s)
            },
            Number(n, _) => {
                let n = *n;
                self.num(n)
            },
            Bool(b, _) => {
                let b = *b;
                self.boolean(b)
            },
            Null(_) => self.null(),
            _ => panic!("Invalid Json")
        }
    }

    fn obj(&mut self) -> Json {
        self.advance();
        
        let mut m = HashMap::new();

        if matches!(self.peek(), RightBrace(_)) {
            self.advance();
            return Json::Obj(Box::new(m));
        }

        let (key, val) = self.member();
        m.insert(key, val);

        while matches!(self.peek(), Comma(_)) {
            self.advance();
            let (key, val) =self.member();
            m.insert(key, val);
        }

        if matches!(self.peek(), RightBrace(_)) {
            self.advance();
        } else {
            panic!("{}", "Invalid token: expected '}'");
        }

        Json::Obj(Box::new(m))
    }

    fn arr(&mut self) -> Json {
        self.advance();

        if matches!(self.peek(), RightBracket(_)) {
            self.advance();
            return Json::Arr(vec![]);
        }

        let mut elements = Vec::new();
        elements.push(self.json());

        while matches!(self.peek(), Comma(_)) {
            self.advance();
            elements.push(self.json());
        }
        
        if matches!(self.peek(), RightBracket(_)) {
            self.advance();
        } else {
            panic!("Invalid token: expected ']");
        }

        Json::Arr(elements)
    }

    fn member(&mut self) -> (String, Json) {
        let key = self.key();

        if matches!(self.peek(), Colon(_)) {
            self.advance();
        } else {
            panic!("Invalid token: expected ':");
        }

        let val = self.json();

        (key, val)
    }

    fn key(&mut self) -> String {
        match &self.advance() {
            String(s, _) => s.to_string(),
            _ => panic!("Invalid token: expected string"),
        }
    }

    fn str(&mut self, s: String) -> Json {
        self.advance();
        Json::Str(s)
    }

    fn num(&mut self, n: f64) -> Json {
        self.advance();
        Json::Num(n)
    }

    fn boolean(&mut self, b: bool) -> Json {
        self.advance();
        Json::Bool(b)
    }

    fn null(&mut self) -> Json {
        self.advance();
        Json::Null
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