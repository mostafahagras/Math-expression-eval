use crate::token::Token;

pub struct Tokenizer<'a> {
    string: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(string: &'a str) -> Self {
        assert_ne!(string.len(), 0, "Cannot evaluate empty expression");
        Self {
            string,
            position: 0,
        }
    }
    pub fn tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
    fn peek(&self) -> Option<char> {
        self.string.chars().nth(self.position)
    }
    fn advance(&mut self) {
        self.position += 1;
    }
    fn consume(&mut self) -> Option<char> {
        let char = self.peek();
        self.advance();
        char
    }
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        match self.peek() {
            Some(character) => match character {
                '+' => {self.advance(); Some(Token::Plus)},
                '-' => {self.advance(); Some(Token::Minus)},
                '*' => {self.advance(); Some(Token::Times)},
                '/' => {self.advance(); Some(Token::Divide)},
                '(' => {self.advance(); Some(Token::LParen)},
                ')' => {self.advance(); Some(Token::RParen)},
                ch if ch.is_digit(10) => Some(self.parse_number()),
                ch => panic!("Unknown character {ch} @ {}", self.position)
            },
            None => None
        }
    }
    fn parse_number(&mut self) -> Token {
        let mut number_string = String::new();
        while let Some(char) = self.peek() {
            if char.is_digit(10) || char == '.' {
                self.advance();
                number_string.push(char);
            } else if char == ',' || char == '_' {
                self.advance();
                continue; // All these are valid: 1,000 or 1_000 or 1000
            } else {
                break;
            }
        }
        let number: f64 = number_string.parse().expect("Failed to ");
        Token::Number(number)
    }
}
