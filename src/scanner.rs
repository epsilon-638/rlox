use crate::token::{ Token, TokenType };

pub struct Scanner<'a> {
    contents: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(contents: &str) -> Scanner {
        let tokens = Vec::new();

        Scanner {
            contents,
            tokens, 
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;  

            match self.scan_token() {
                Some(token) => self.tokens.push(token),
                None => (),
            };
        }

        let eof = Token::new(
            TokenType::EOF, 
            String::from(""), 
            self.line,
        );

        self.tokens.push(eof);

        Vec::new()
    }
    pub fn scan_token(&mut self) -> Option<Token> {
        match self.advance() {
            '(' => Some(Token::new(TokenType::LeftParen, String::from(""), self.line)),
            ')' => Some(Token::new(TokenType::RightParen, String::from(""), self.line)),
            '{' => Some(Token::new(TokenType::LeftBrace, String::from(""), self.line)),
            '}' => Some(Token::new(TokenType::RightBrace, String::from(""), self.line)),
            ',' => Some(Token::new(TokenType::Comma, String::from(""), self.line)),
            '.' => Some(Token::new(TokenType::Dot, String::from(""), self.line)),
            '-' => Some(Token::new(TokenType::Minus, String::from(""), self.line)),
            '+' => Some(Token::new(TokenType::Plus, String::from(""), self.line)),
            ';' => Some(Token::new(TokenType::Semicolon, String::from(""), self.line)),
            '*' => Some(Token::new(TokenType::Star, String::from(""), self.line)),
            '\n' => self.new_line(),
            _ => None,
       }
    }
    pub fn new_line(&mut self) -> Option<Token> {
        self.line += 1;
        None
    }
    pub fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;

        self.contents
            .chars()
            .nth(current)
            .unwrap()
    }
    pub fn get_contents(&self) -> &str {
        self.contents.clone()
    }
    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.contents.len()
    }
}
