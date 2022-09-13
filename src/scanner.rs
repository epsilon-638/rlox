use crate::token::Token;

pub struct Scanner<'a> {
    contents: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(contents: &str) -> Scanner {
        Scanner {
            contents,
        }
    }
    pub fn scan_tokens(&self) -> Vec<Token> {
        unimplemented!()
    }
    pub fn get_contents(&self) -> &str {
        self.contents.clone()
    }
}
