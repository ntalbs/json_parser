#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(std::string::String),
    Number(f64),
    Bool(bool),
    Null,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

#[derive(Debug)]
pub struct Error {
    message: String,
    line: usize,
    pos: usize,
}

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    pub errors: Vec<Error>,
    start: usize,
    current: usize,
    line: usize,
    pos: usize,
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
        Scanner {
            source: input,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            pos: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::EOF);

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ':' => self.add_token(TokenType::Colon),
            ',' => self.add_token(TokenType::Comma),
            ' '|'\t'|'\r' => {},
            '\n' => {
                self.line += 1;
                self.pos = 0;
            }
            '"' => self.string(),
            '-'|'0'..='9' => self.number(),
            _ => {
                if self.is_alpha(c) {
                    self.keyword();
                } else {
                    self.add_error("Unexpected token.".to_string());
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let t = Token {
            token_type,
            lexeme: self.source[self.start..self.current].to_string()
        };
        self.tokens.push(t);
    }

    fn add_error(&mut self, message: String) {
        let e = Error {
             message,
             line: self.line,
            pos: self.pos,
        };
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
        self.pos += 1;
        self.current += 1;
        self.source.chars().nth(self.current-1).unwrap()
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None
        } else {
            self.source.chars().nth(self.current)
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            self.source.chars().nth(self.current + 1)
        }
    }

    fn string(&mut self) {
        while self.peek().unwrap() != '"' && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() {
            self.add_error("Unterminated string".to_string());
        }
        self.advance();
        let value = self.source[self.start+1..self.current-1].to_string();
        self.add_token(TokenType::String(value));
    }

    fn number(&mut self) {
        if self.peek().unwrap() == '-' {
            self.advance();
        }
        while self.is_digit(self.peek().unwrap()) {
            self.advance();
        }
        if self.peek().unwrap() == '.' && self.peek_next().unwrap().is_digit(10) {
            self.advance();
        }
        while self.peek().unwrap().is_digit(10) {
            self.advance();
        }
        let value = self.source[self.start..self.current].parse::<f64>().unwrap();
        self.add_token(TokenType::Number(value));
    }

    fn keyword(&mut self) {
        while self.peek().unwrap().is_alphabetic() {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match text {
            "true" => self.add_token(TokenType::Bool(true)),
            "false" => self.add_token(TokenType::Bool(false)),
            "null" => self.add_token(TokenType::Null),
            _ => {
                let err_message = format!("{}: {}", "Unexpected token", text);
                self.add_error(err_message);
            }
        }
    }
}
