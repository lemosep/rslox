use crate::{lox_error, literal};
use crate::token::{Token, TokenType};
use crate::literal::Value;

/// Takes raw source code as a series of characters and groups it into tokens.
pub struct Scanner {
    /// The raw source code.
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    /// Current token being iterated
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
            // Also lexemes, but they do not produce any value
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            //string literals
            '"' => self.string(),
            _ => lox_error::error(self.line, "Unexpected character"),
        }
    }

    fn add_token(&mut self, token: TokenType) -> () {
        self.push_token(token, None)
    }

    // Is this even performance-prone?
    fn add_string_lexeme(&mut self, token: TokenType, literal: literal::Value) -> () {
     
    }

    fn push_token(&mut self, token: TokenType, literal: Option<literal::Value>) -> () {
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
        match self.match_char('/') {
            true => {
                while (self.peek() != '\n') && !self.is_at_end() {
                    self.advance();
                }
            },
            false => self.add_token(TokenType::Slash),
        }
    }

    /// Returns the current character without consuming it.
    fn peek(&self) -> char {
        match self.is_at_end() {
            true => '\0',
            false => self.source.chars().nth(self.current).unwrap().clone(),
        }
    }

    /// Treats string literals
    fn string(&mut self) {
        while (self.peek() != '\n') && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            lox_error::error(self.line, "Unterminated string.");
            return;
        }   

        // Closing ".
        self.advance();

        // Trimming surrounding quotes.
        let val = self.source.get(self.start..(self.current - 1)).unwrap();
        
        self.add_string_lexeme(
            TokenType::String,
            literal::Value::Str { v: val.to_string() }
        );
    }  

    /// Verify if source has reached EOF.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
