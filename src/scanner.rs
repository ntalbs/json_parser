use crate::construct::{Error, Pos, Token};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    errors: Vec<Error>,
    start: usize,
    current: usize,
    pos: Pos,
}

impl <'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            source: input,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            pos: Pos { line: 1, col: 1 }
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, &Vec<Error>> {
        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token();
            match token {
                Ok(t) => self.add_token(t),
                Err(e) => self.add_error(e),
            };
        }
        self.add_token(Token::EOF);

        if self.errors.is_empty() {
            Result::Ok(&self.tokens)
        } else {
            Err(&self.errors)
        }
    }

    fn scan_token(&mut self) -> Result<Token<'a>, Error> {
        let c = self.skip_whitespaces();
        match c {
            '{' => Ok(Token::LeftBrace {
                lexeme: "{",
                pos: self.pos
            }),
            '}' => Ok(Token::RightBrace {
                lexeme: "}",
                pos: self.pos
            }),
            '[' => Ok(Token::LeftBracket {
                lexeme: "[",
                pos: self.pos
            }),
            ']' => Ok(Token::RightBracket {
                lexeme: "]",
                pos: self.pos
            }),
            ':' => Ok(Token::Colon {
                lexeme: ":",
                pos: self.pos
            }),
            ',' => Ok(Token::Comma {
                lexeme: ",",
                pos: self.pos
            }),
            '"' => self.string(),
            '-'|'0'..='9' => self.number(),
            _ => {
                if self.is_alpha(c) {
                    self.keyword()
                } else {
                    Err(Error {
                        message: "Unexpected token.".to_string(),
                        pos: self.pos,
                    })
                }
            }
        }
    }

    fn skip_whitespaces(&mut self) -> char {
        loop {
            let c = self.advance();
            match c {
                ' ' | '\t' => {
                    self.start = self.current;
                },
                '\n' | '\r' => {
                    self.start = self.current;
                    self.pos.line += 1;
                    self.pos.col = 1;    
                }
                _ => {
                    return c;
                }
            }
        }
    }

    fn add_token(&mut self, t: Token<'a>) {
        self.tokens.push(t);
    }

    fn add_error(&mut self, e: Error) {
        self.errors.push(e);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic()
    }

    fn advance(&mut self) -> char {
        self.pos.col += 1;
        self.current += 1;
        self.source.chars().nth(self.current-1).unwrap()
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<Token<'a>, Error> {
        loop {
            let c = self.peek();
            if c == '"' {  // end of string
                break;
            }
            if c == '\n' || self.is_at_end() {
                return Err(Error {
                    message: "Unterminated string".to_string(),
                    pos: self.pos,
                })
            }
            self.advance();
        }

        self.advance();
        let lexeme = &self.source[self.start .. self.current]; 
        let val= &self.source[self.start + 1 .. self.current - 1];
        
        Ok(Token::String {
            lexeme,
            val,
            pos: self.pos,
        })
    }

    fn number(&mut self) -> Result<Token<'a>, Error> {
        if self.peek() == '-' {
            self.advance();
        }
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
        }
        while self.peek().is_digit(10) {
            self.advance();
        }
        let lexeme = &self.source[self.start..self.current];
        println!(">>>|{}|", lexeme);
        let val = lexeme.parse::<f64>().unwrap();
        Ok(Token::Number {
            lexeme,
            val,
            pos: self.pos
        })
    }

    fn keyword(&mut self) -> Result<Token<'a>, Error> {
        while self.peek().is_alphabetic() {
            self.advance();
        }
        let lexeme = &self.source[self.start..self.current];
        match lexeme {
            "true" | "false" => Ok(Token::Bool {
                lexeme,
                val: lexeme.parse::<bool>().unwrap(),
                pos: self.pos
            }),
            "null" => Ok(Token::Null {
                lexeme,
                pos: self.pos
            }),
            _ => {
                Err(Error {
                    message: format!("{}: {}", "Unexpected token", lexeme),
                    pos: self.pos,
               })
            }
        }
    }
}
