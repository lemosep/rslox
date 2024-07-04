#[derive(Debug)]
pub enum TokenType {
    /// Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    /// One or two-character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    /// Literals
    Identifier,
    String,
    Number,

    /// Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    fn to_string(&self) -> String {
        let literal_str = match &self.literal {
            Some(literal) => literal.clone(),
            None => "null".to_string(),
        };
        format!("{:?} {} {}", self.token_type, self.lexeme, literal_str)
    }
}
