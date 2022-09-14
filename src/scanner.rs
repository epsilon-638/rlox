use crate::error::{ error, ErrorType, parser_error };
use crate::token::{ Token, TokenType };
use std::collections::HashMap;

pub struct Scanner<'a> {
    contents: &'a str,
    tokens: Vec<Token>,
    reserved_words: HashMap<&'a str, TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(contents: &str) -> Scanner {
        let tokens = Vec::new();

        let reserved_words = HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ]);

        Scanner {
            contents,
            tokens,
            reserved_words,
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
    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
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
            '\n' => self.new_line_token(),
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
            },
            '/' => {
                match self.match_char('/') {
                    true => {
                        while self.peek_char() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                        // Comments are removed during lexing

                        None
                    },
                    false => self.new_token(TokenType::Slash),
                }
            },
            ' ' => None,
            '\r' => None,
            '\t' => None,
            '"' => self.string_token(),
            '0' ..= '9' => self.number_token(),
            'a' ..= 'z' | '_' => self.identifier_token(),
            _ => {
                parser_error(self.line, ErrorType::UnexpectedChar);

                None
            },
       }
    }
    fn new_line(&mut self) {
        self.line += 1;
    }
    fn new_line_token(&mut self) -> Option<Token> {
        self.new_line();
        None
    }
    fn string_token(&mut self) -> Option<Token> {
        while self.peek_char() != '"' && !self.is_at_end() {
            if self.peek_char() == '\n' {
                self.new_line();
            }

            self.advance();
        }
        
        if self.is_at_end() {
            error(self.line, "Unterminated String");

            return None;
        }

        self.advance();

        let value = &self.contents[self.start+1..self.current-1];

        self.new_token(TokenType::String(value.to_string()))
    }
    fn number_token(&mut self) -> Option<Token> {
        while self.is_digit(self.peek_char()) {
            self.advance();
        }

        if self.peek_char() == '.' && self.is_digit(self.peek_next_char()) {
            self.advance();

            while self.is_digit(self.peek_char()) {
                self.advance();
            }
        }

        let value = &self.contents[self.start..self.current]
            .trim()
            .parse::<i64>()
            .unwrap();

        self.new_token(TokenType::Number(value.clone()))
    }
    fn identifier_token(&mut self) -> Option<Token> {
        while self.is_alphanumeric(self.peek_char()) {
            self.advance();
        }

        let value = &self.contents[self.start..self.current];

        match self.reserved_words.get(value) {
            Some(identifier) => {
                self.new_token(identifier.clone())
            },
            None => {
                self.new_token(TokenType::Identifier(value.to_string()))
            },
        }
    }
    fn new_token(&self, token_type: TokenType) -> Option<Token> {
        Some(
            Token::new(
                token_type,
                String::from(""),
                self.line,
            ),
        )
    }
    fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;

        self.contents
            .chars()
            .nth(current)
            .unwrap()
    }
    fn match_char(&mut self, expected: char) -> bool {
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
    fn peek_char(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.contents
            .chars()
            .nth(self.current)
            .unwrap()
    }
    fn peek_next_char(&self) -> char {
        if self.current + 1 >= self.contents.len() {
            return '\0';
        }

        self.contents
            .chars()
            .nth(self.current)
            .unwrap()
    }
    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && 
        c <= 'z' || 
        c == '_'
    }
    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.contents.len()
    }
    pub fn get_contents(&self) -> &str {
        self.contents.clone()
    }
    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}
