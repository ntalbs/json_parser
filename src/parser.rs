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
            String(s) => {
                let s = s.to_string();
                self.str(s)
            },
            Number(n) => {
                let n = *n;
                self.num(n)
            },
            Bool(b) => {
                let b = *b;
                self.boolean(b)
            },
            Null => self.null(),
            _ => panic!("Invalid Json")
        }
    }

    fn obj(&mut self) -> Json {
        self.advance();
        
        let mut m = HashMap::new();

        if matches!(self.peek(), RightBrace) {
            self.advance();
            return Json::Obj(Box::new(m));
        }

        let (key, val) = self.member();
        m.insert(key, val);

        while matches!(self.peek(), Comma) {
            self.advance();
            let (key, val) =self.member();
            m.insert(key, val);
        }

        if matches!(self.peek(), RightBrace) {
            self.advance();
        } else {
            panic!("{}", "Invalid token: expected '}'");
        }

        Json::Obj(Box::new(m))
    }

    fn arr(&mut self) -> Json {
        self.advance();

        if matches!(self.peek(), RightBracket) {
            self.advance();
            return Json::Arr(vec![]);
        }

        let mut elements = Vec::new();
        elements.push(self.json());

        while matches!(self.peek(), Comma) {
            self.advance();
            elements.push(self.json());
        }
        
        if matches!(self.peek(), RightBracket) {
            self.advance();
        } else {
            panic!("Invalid token: expected ']");
        }

        Json::Arr(elements)
    }

    fn member(&mut self) -> (String, Json) {
        let key = self.key();

        if matches!(self.peek(), Colon) {
            self.advance();
        } else {
            panic!("Invalid token: expected ':");
        }

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