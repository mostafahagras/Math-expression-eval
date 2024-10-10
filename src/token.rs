use crate::ast::Operator;

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Times,
    Divide,
    LParen,
    RParen,
    Number(f64),
    // EOF,
}

impl Token {
    /// Returns `true` if the token is [`Number`].
    ///
    /// [`Number`]: Token::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    pub fn value(&self) -> f64 {
        match self {
            Self::Number(n) => *n,
            _ => panic!("Token::value called on a non-number token")
        }
    }

    /// Returns `true` if the token is [`Plus`], `Minus`, `Times`, `Divide`].
    ///
    /// [`Plus`, `Minus`, `Times`, `Divide`]: Token::Plus, Token::Minus, Token::Times, Token::Divide
    #[must_use]
    pub fn is_operator(&self) -> bool {
        matches!(self, Self::Plus | Self::Minus | Self::Times | Self::Divide)
    }

    pub fn operator(&self) -> Operator {
        match self {
            Self::Plus => Operator::Add,
            Self::Minus => Operator::Subtract,
            Self::Times => Operator::Multiply,
            Self::Divide => Operator::Divide,
            _ => panic!("Token::operator called with a non-operator token")
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Token::Plus | Token::Minus => 0,
            Token::Times | Token::Divide => 1,
            _ => unreachable!("Found {self:?} in Token::precedence"),
        }
    }

    /// Returns `true` if the token is [`LParen`].
    ///
    /// [`LParen`]: Token::LParen
    #[must_use]
    pub fn is_lparen(&self) -> bool {
        matches!(self, Self::LParen)
    }

    /// Returns `true` if the token is [`RParen`].
    ///
    /// [`RParen`]: Token::RParen
    #[must_use]
    pub fn is_rparen(&self) -> bool {
        matches!(self, Self::RParen)
    }
}
