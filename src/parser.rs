use std::collections::HashMap;
use std::string::String;

use crate::scanner::Token;
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
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Json {
        self.json()
    }

    fn json(&mut self) -> Json {
        let token = self.peek();
        match token {
            LeftBrace => self.obj(),
            LeftBracket => self.arr(),
            String(s) => self.str(s.to_string()),
            Number(n) => self.num(*n),
            Bool(b) => self.boolean(*b),
            Null => self.null(),
            _ => panic!("Invalid Json")
        }
    }

    fn obj(&mut self) -> Json {
        self.advance();
        
        let mut m = HashMap::new();

        if self.check(RightBrace) {
            self.advance();
            return Json::Obj(Box::new(m));
        }

        let (key, val) = self.member();
        m.insert(key, val);

        while self.check(Comma) {
            self.advance();
            let (key, val) =self.member();
            m.insert(key, val);
        }

        self.consume(RightBrace, "Invalid token: expected '}".to_string());
        Json::Obj(Box::new(m))
    }

    fn arr(&mut self) -> Json {
        self.advance();

        if self.check(RightBracket) {
            self.advance();
            return Json::Arr(vec![]);
        }

        let mut elements = Vec::new();
        elements.push(self.json());

        while self.check(Comma) {
            self.advance();
            elements.push(self.json());
        }
        
        self.consume(RightBracket, "invalid token: expected ']'".to_string());
        Json::Arr(elements)
    }

    fn member(&mut self) -> (String, Json) {
        let key = self.key();
        self.consume(Colon, "Invalid token: expected ':'".to_string());
        let val = self.json();

        (key, val)
    }

    fn key(&mut self) -> String {
        match &self.advance() {
            String(s) => s.to_string(),
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

    fn check(&self, token: Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        *self.peek() == token
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

    fn consume(&mut self, token: Token, message: String) -> &Token {
        if self.check(token) {
            return self.advance();
        }
        panic!("{}", message);
    }
}