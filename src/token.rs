#[derive(Debug)]
pub enum TokenType {
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
    Bang,
    BangEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String(String),
    Number(i64),
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
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: i32,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
    pub fn to_string(&self) -> String {
        let literal = match &self.token_type {
            TokenType::String(l) => {
                l.clone()
            },
            TokenType::Number(l) => {
                l.to_string()
            },
            _ => String::from(""),
        };

        format!(
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            literal,
        )
    }
}
