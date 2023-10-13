use crate::{Error, Pos, Token};

pub(crate) struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    errors: Vec<Error>,
    start: usize,
    current: usize,
    pos: Pos,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Scanner {
            source: input,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            pos: Pos { line: 1, col: 0 },
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> Result<&[Token], &[Error]> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(Token::Eof);

        if self.errors.is_empty() {
            Ok(&self.tokens)
        } else {
            Err(&self.errors)
        }
    }

    fn scan_token(&mut self) {
        let o = self.advance();
        let c = match o {
            None => return,
            Some(c) => c,
        };
        match c {
            '{' => self.add_token(Token::LeftBrace(self.pos)),
            '}' => self.add_token(Token::RightBrace(self.pos)),
            '[' => self.add_token(Token::LeftBracket(self.pos)),
            ']' => self.add_token(Token::RightBracket(self.pos)),
            ':' => self.add_token(Token::Colon(self.pos)),
            ',' => self.add_token(Token::Comma(self.pos)),
            ' ' | '\t' => {}
            '\n' | '\r' => {
                self.pos.line += 1;
                self.pos.col = 0;
            }
            '"' => self.string(),
            '-' | '0'..='9' => self.number(),
            _ => {
                if self.is_alpha(c) {
                    self.keyword();
                } else {
                    self.add_error("Unexpected token.".to_string());
                }
            }
        }
    }

    fn add_token(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    fn add_error(&mut self, message: String) {
        let e = Error {
            message,
            pos: self.pos,
        };
        self.errors.push(e);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic()
    }

    fn is_alnum(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.pos.col += 1;
            self.current += 1;
            Some(self.source.chars().nth(self.current - 1).unwrap())
        }
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.source.chars().nth(self.current)
        }
    }

    fn string(&mut self) {
        let mut escape = false;
        loop {
            match self.peek() {
                Some('\\') => escape = true,
                Some('"') => {
                    if !escape {
                        break;
                    } else {
                        escape = false;
                    }
                }
                Some('\n') | None => {
                    self.add_error("Unterminated string".to_string());
                    break;
                }
                Some(_) => {}
            }
            self.advance();
        }

        self.advance();
        let lexeme = &self.source[self.start..self.current];
        let val = &self.source[self.start + 1..self.current - 1];
        self.add_token(Token::String {
            lexeme,
            val,
            pos: self.pos,
        });
    }

    fn number(&mut self) {
        self.digits();

        if let Some('.') = self.peek() {
            self.advance();
            self.digits();
        }

        let lexeme = &self.source[self.start..self.current];
        let val = lexeme.parse::<f64>().unwrap();
        self.add_token(Token::Number {
            lexeme,
            val,
            pos: self.pos,
        });
    }

    fn digits(&mut self) {
        while let Some(c) = self.peek() {
            if self.is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn keyword(&mut self) {
        while let Some(c) = self.peek() {
            if self.is_alnum(c) {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[self.start..self.current];
        match lexeme {
            "true" | "false" => self.add_token(Token::Bool {
                lexeme,
                val: lexeme.parse::<bool>().unwrap(),
                pos: self.pos,
            }),
            "null" => self.add_token(Token::Null {
                lexeme,
                pos: self.pos,
            }),
            _ => {
                let err_message = format!("{}: {}", "Unexpected token", lexeme);
                self.add_error(err_message);
            }
        }
    }
}
