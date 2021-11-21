#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pos {
    line: usize,
    col: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace(Pos),
    RightBrace(Pos),
    LeftBracket(Pos),
    RightBracket(Pos),
    Colon(Pos),
    Comma(Pos),
    String(std::string::String, Pos),
    Number(f64, Pos),
    Bool(bool, Pos),
    Null(Pos),
    EOF,
}

#[derive(Debug)]
pub struct Error {
    message: String,
    pos: Pos,
}

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    pub errors: Vec<Error>,
    start: usize,
    current: usize,
    pos: Pos,
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
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
            self.scan_token();
        }
        self.add_token(Token::EOF);

        if self.errors.len() > 0 {
            Err(&self.errors)
        } else {
            Result::Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '{' => self.add_token(Token::LeftBrace(self.pos)),
            '}' => self.add_token(Token::RightBrace(self.pos)),
            '[' => self.add_token(Token::LeftBracket(self.pos)),
            ']' => self.add_token(Token::RightBracket(self.pos)),
            ':' => self.add_token(Token::Colon(self.pos)),
            ',' => self.add_token(Token::Comma(self.pos)),
            ' '|'\t' => {},
            '\n'|'\r' => {
                self.pos.line += 1;
                self.pos.col = 1;
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

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn add_error(&mut self, message: String) {
        let e = Error {
             message,
             pos: Pos {
                line: self.pos.line,
                col: self.pos.col,
             },
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
        self.pos.col += 1;
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
        self.add_token(Token::String(value, self.pos));
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
        self.add_token(Token::Number(value, self.pos));
    }

    fn keyword(&mut self) {
        while self.peek().unwrap().is_alphabetic() {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match text {
            "true" => self.add_token(Token::Bool(true, self.pos)),
            "false" => self.add_token(Token::Bool(false, self.pos)),
            "null" => self.add_token(Token::Null(self.pos)),
            _ => {
                let err_message = format!("{}: {}", "Unexpected token", text);
                self.add_error(err_message);
            }
        }
    }
}
