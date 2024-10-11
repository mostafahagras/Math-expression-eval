#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Number(f64),
    BinaryOp(Box<AstNode>, Operator, Box<AstNode>),
    UnaryOp(UnaryOp, Box<AstNode>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Positive,
    Negative,
    // Other things like factorial, functions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    // Other things like exponentiation, logarithms
}

impl UnaryOp {
    pub fn eval(&self, n: &Box<AstNode>) -> f64 {
        match self {
            UnaryOp::Positive => n.eval(),
            UnaryOp::Negative => -n.eval(),
        }
    }
}

impl Operator {
    pub fn eval(&self, lhs: &Box<AstNode>, rhs: &Box<AstNode>) -> f64 {
        match self {
            Operator::Add => lhs.eval() + rhs.eval(),
            Operator::Subtract => lhs.eval() - rhs.eval(),
            Operator::Multiply => lhs.eval() * rhs.eval(),
            Operator::Divide => lhs.eval() / rhs.eval(),
        }
    }
}

impl AstNode {
    pub fn eval(&self) -> f64 {
        match self {
            AstNode::BinaryOp(lhs, op, rhs) => op.eval(lhs, rhs),
            AstNode::Number(number) => *number,
            AstNode::UnaryOp(unary_op, number) => unary_op.eval(number),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, tokenizer::Tokenizer};

    #[test]
    fn test() {
        let expressions = vec![
            "1 + 1",
            "2 * 3",
            "4 - 2",
            "6 / 3",
            "2 + 3 * 4",
            "10 / 2 - 1",
            "2(3 + 4)",
            "(2 + 3) * 4",
            "(1 - 2) * (3 + 4)",
            "2(2)(3)",
            "(2 + 3)(4 - 1)",
            "2 + (3 * 4) - (1 / 2)",
            "3 * (2 + 1)",
            "(5 - 2) * (1 + 2) / 3",
            "2 + (4 * 2) * 2",
            "(2)(2)",
            "2(2 + 3)",
            "(2 - 1)(3 + 2)",
            "-3 + 5",
            "1 + 2 * 3 - 4 / 5",
        ];
        let expected_results = vec![
            2.0, 6.0, 2.0, 2.0, 14.0, 4.0, 14.0, 20.0, -7.0, 12.0, 15.0, 13.5, 9.0, 3.0, 18.0, 4.0,
            10.0, 5.0, 2.0, 6.2,
        ];
        for i in 0..20 {
            let mut tokenizer = Tokenizer::new(expressions[i]);
            let tokens = tokenizer.tokens();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse();
            let result = ast.eval();
            assert_eq!(result, expected_results[i]);
        }
    }
}
