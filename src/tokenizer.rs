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
                '+' => {
                    self.advance();
                    Some(Token::Plus)
                }
                '-' => {
                    self.advance();
                    Some(Token::Minus)
                }
                '*' => {
                    self.advance();
                    Some(Token::Times)
                }
                '/' => {
                    self.advance();
                    Some(Token::Divide)
                }
                '(' => {
                    self.advance();
                    Some(Token::LParen)
                }
                ')' => {
                    self.advance();
                    Some(Token::RParen)
                }
                ch if ch.is_digit(10) => Some(self.parse_number()),
                ch => panic!("Unknown character {ch} @ {}", self.position),
            },
            None => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokens()
    }

    #[test]
    fn test_tokenize_simple_expression() {
        let input = "1 + 2";
        let expected = vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_with_parentheses() {
        let input = "(1 + 2) * 3";
        let expected = vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
            Token::Times,
            Token::Number(3.0),
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_with_division_and_subtraction() {
        let input = "10 / 2 - 3";
        let expected = vec![
            Token::Number(10.0),
            Token::Divide,
            Token::Number(2.0),
            Token::Minus,
            Token::Number(3.0),
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_implicit_multiplication_number_and_parenthesis() {
        let input = "2(3 + 4)";
        let expected = vec![
            Token::Number(2.0),
            Token::LParen,
            Token::Number(3.0),
            Token::Plus,
            Token::Number(4.0),
            Token::RParen,
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_implicit_multiplication_between_parentheses() {
        let input = "(2)(3 + 4)";
        let expected = vec![
            Token::LParen,
            Token::Number(2.0),
            Token::RParen,
            Token::LParen,
            Token::Number(3.0),
            Token::Plus,
            Token::Number(4.0),
            Token::RParen,
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let input = "3 + 4 * 2 / (1 - 5)";
        let expected = vec![
            Token::Number(3.0),
            Token::Plus,
            Token::Number(4.0),
            Token::Times,
            Token::Number(2.0),
            Token::Divide,
            Token::LParen,
            Token::Number(1.0),
            Token::Minus,
            Token::Number(5.0),
            Token::RParen,
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_implicit_multiplication_complex_case() {
        let input = "(1 + 2)(3 + 4)";
        let expected = vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
            Token::LParen,
            Token::Number(3.0),
            Token::Plus,
            Token::Number(4.0),
            Token::RParen,
        ];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_decimal_numbers() {
        let input = "1.5 + 2.25";
        let expected = vec![Token::Number(1.5), Token::Plus, Token::Number(2.25)];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_negative_numbers() {
        let input = "-3 + 2";
        let expected = vec![
            Token::Minus, // Unary minus
            Token::Number(3.0),
            Token::Plus,
            Token::Number(2.0),
        ];
        assert_eq!(tokenize(input), expected);
    }
}
