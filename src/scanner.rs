use std::intrinsics::rustc_peek;

use crate::token::{
    Token,
    TokenType::{self, *},
};

use crate::error;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    /// Creates new Scanner instance
    pub fn new(src: String) -> Self {
        Scanner {
            source: src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        let mut c = self.advance();

        match c {
            Some('(') => self.add_token(LeftParen),
            Some(')') => self.add_token(RightParen),
            Some('{') => self.add_token(LeftBrace),
            Some('}') => self.add_token(RightBrace),
            Some(',') => self.add_token(Comma),
            Some('.') => self.add_token(Dot),
            Some('-') => self.add_token(Minus),
            Some('+') => self.add_token(Plus),
            Some(';') => self.add_token(Semicolon),
            Some('*') => self.add_token(Star),
            Some('!') => self.add_token(if self.match_char('=') {
                BangEqual
            } else {
                Bang
            }),
            Some('=') => self.add_token(if self.match_char('=') {
                EqualEqual
            } else {
                Equal
            }),
            Some('<') => self.add_token(if self.match_char('=') {
                LessEqual
            } else {
                Less
            }),
            Some('>') => self.add_token(if self.match_char('=') {
                GreaterEqual
            } else {
                Greater
            }),
            Some('/') => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while (self.peek() != Some('\n')) && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            Some('"') => self.handle_strings(),
            Some('\n') => self.line += 1,
            Some(' ') | Some('\r') | Some('\t') => {} // Ignore whitespace.
            Some(_) | None => error::error(self.line as i32, "Unexpected character.".to_string()),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let result = self.source.chars().nth(self.current);
        self.current += 1;
        result
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, None, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) == Some(expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn peek(&mut self) -private void addToken(TokenType type, Object literal) {
        String text = source.substring(start, current);
        tokens.add(new Token(type, text, literal, line));
      }
    > Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.source.chars().nth(self.current)
        }
    }

    fn handle_strings(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.linprivate void addToken(TokenType type, Object literal) {
                    String text = source.substring(start, current);
                    tokens.add(new Token(type, text, literal, line));
                  }
                e += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error::error(self.line as i32, "Unterminated string.".to_string());
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(String, value);
    }
}
