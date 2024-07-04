use crate::lox_error;
use crate::token::{Token, TokenType};

/// Takes raw source code as a series of characters and groups it into tokens.
pub struct Scanner {
    /// The raw source code.
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    #[must_use]
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Loops over the source code adding tokens.
    pub fn scan_tokens(&mut self) -> eyre::Result<()> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            crate::token::TokenType::Eof,
            "".to_string(),
            None,
            self.line,
        ));

        Ok(())
    }

    /// Consumes a single character and analyzes it's `TokenType`
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            // Comparison for two-sized lexemes
            '!' => self.check_second_char('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.check_second_char('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.check_second_char('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.check_second_char('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => self.handle_slash(),
            _ => lox_error::error(self.line, "Unexpected character"),
        }
    }

    fn add_token(&mut self, token: TokenType) -> () {
        self.push_token(token, None)
    }

    fn push_token(&mut self, token: TokenType, literal: Option<String>) -> () {
        let lexeme = String::from(&self.source[self.start..self.current]);
        self.tokens
            .push(Token::new(token, lexeme, literal, self.line))
    }

    /// Consumes the next character in the source file.
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    /// Checks for lexemes containing more than one character.
    fn check_second_char(&mut self, expected: char, t1: TokenType, t2: TokenType) {
        if self.match_char(expected) {
            self.add_token(t1);
        } else {
            self.add_token(t2);
        }
    }

    /// Returns true if the next character corresponds to the @param:expected.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let curr = self.source.chars().nth(self.current).unwrap();
        if curr != expected {
            return false;
        }
        self.current += 1;
        true
    }

    /// Specifically for cases containing '/' character.
    fn handle_slash(&mut self) {
        if self.match_char('/') {
            while self.peek {}
        }
    }

    /// Returns the current character without consuming it.
    fn peek(&self) {}

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
