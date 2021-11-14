#[derive(Debug)]
enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String,
    Number,
    True,
    False,
    Null,
    EOF,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    literal: Option<String>,
}

#[derive(Debug)]
struct Error {
    message: String,
    line: usize,
    pos: usize,
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<Error>,
    start: usize,
    current: usize,
    line: usize,
    pos: usize,
}

impl Scanner {
    fn new(input: String) -> Scanner {
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

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

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
            token_type: token_type,
            lexeme: None,
            literal: None
        };
        self.tokens.push(t);
    }

    fn add_token_with_value(&mut self, token_type: TokenType, literal: Option<String>) {
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: Some(self.source[self.start..self.current].to_string()),
            literal: literal
        });
    }

    fn add_error(&mut self, message: String) {
        let e = Error {
             message: message,
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
        self.add_token_with_value(TokenType::String, Some(value));
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
        let value = self.source[self.start..self.current].to_string();
        self.add_token_with_value(TokenType::Number, Some(value));
    }

    fn keyword(&mut self) {
        while self.peek().unwrap().is_alphabetic() {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match text {
            "true" => self.add_token(TokenType::True),
            "false" => self.add_token(TokenType::False),
            "null" => self.add_token(TokenType::Null),
            _ => self.add_error(format!("{}: {}", "Unexpected token", text)),
        }
    }
}

fn main() {
    let input = r#"
    {
        "a": 10,
        "b": true,
        "c": "hello",
        "d": [1, 2, 3],
        "e": {
            "e1": true,
            "e2": false
        },
        "f": [
            {"a": 10},
            {"a": 20}
        ]
    }
    "#;

    let mut scanner = Scanner::new(input.to_string());

    for t in scanner.scan_tokens() {
        println!("{:?}", t);
    }
}