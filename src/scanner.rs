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
            '(' => self.new_token(TokenType::LeftParen),
            ')' => self.new_token(TokenType::RightParen),
            '{' => self.new_token(TokenType::LeftBrace),
            '}' => self.new_token(TokenType::RightBrace),
            ',' => self.new_token(TokenType::Comma),
            '.' => self.new_token(TokenType::Dot),
            '-' => self.new_token(TokenType::Minus),
            '+' => self.new_token(TokenType::Plus),
            ';' => self.new_token(TokenType::Semicolon),
            '*' => self.new_token(TokenType::Star),
            '\n' => self.new_line(),
            '!' => {
                match self.match_char('=') {
                    true => self.new_token(TokenType::BangEqual),
                    false => self.new_token(TokenType::Bang),
                }
            },
            '=' => {
                match self.match_char('=') {
                    true => self.new_token(TokenType::EqualEqual),
                    false => self.new_token(TokenType::Equal),
                }
            },
            '<' => {
                match self.match_char('=') {
                    true => self.new_token(TokenType::LessEqual),
                    false => self.new_token(TokenType::Less),
                }
            },
            '>' => {
                match self.match_char('=') {
                    true => self.new_token(TokenType::GreaterEqual),
                    false => self.new_token(TokenType::GreaterEqual),
                }
            }
            _ => None,
       }
    }
    pub fn new_line(&mut self) -> Option<Token> {
        self.line += 1;
        None
    }
    pub fn new_token(&self, token_type: TokenType) -> Option<Token> {
        Some(
            Token::new(
                token_type,
                String::from(""),
                self.line,
            ),
        )
    }
    pub fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;

        self.contents
            .chars()
            .nth(current)
            .unwrap()
    }
    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.contents
            .chars()
            .nth(self.current)
            .unwrap();

        if next_char != expected {
            return false;
        }

        self.current += 1;

        true
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
