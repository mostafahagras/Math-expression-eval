use crate::{ast::AstNode, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    // position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            // position: 0,
        }
    }
    pub fn parse(&mut self) -> AstNode {
        let mut operands: Vec<AstNode> = vec![];
        let mut operators: Vec<&Token> = vec![];
        for token in &self.tokens {
            if token.is_number() {
                operands.push(AstNode::Number(token.value()));
            } else if token.is_operator() {
                while let Some(last_operator) = operators.last() {
                    if last_operator.is_lparen() {
                        break;
                    }
                    if last_operator.precedence() >= token.precedence() {
                        let right = operands.pop().expect("No right operand");
                        let left = operands.pop().expect("No left operand");
                        let op = operators.pop().expect("No operator");
                        let ast_node =
                            AstNode::BinaryOp(Box::new(left), op.operator(), Box::new(right));
                        operands.push(ast_node);
                    } else {
                        break;
                    }
                }
                operators.push(token);
            } else if token.is_lparen() {
                operators.push(token);
            } else if token.is_rparen() {
                while let Some(last_operator) = operators.pop() {
                    if last_operator.is_lparen() {
                        break;
                    }
                    let right = operands.pop().expect("No right operand");
                    let left = operands.pop().expect("No left operand");
                    let ast_node = AstNode::BinaryOp(
                        Box::new(left),
                        last_operator.operator(),
                        Box::new(right),
                    );
                    operands.push(ast_node);
                }
            } else {
                unimplemented!("For additional features")
            }
        }
        while let Some(operator) = operators.pop() {
            let right = operands.pop().expect("No right operand");
            let left = operands.pop().expect("No left operand");
            let ast_node = AstNode::BinaryOp(Box::new(left), operator.operator(), Box::new(right));
            operands.push(ast_node);
        }
        operands.pop().expect("Failed to parse expression")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::Operator, tokenizer::Tokenizer};
    fn parse_expression(input: &str) -> AstNode {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokens();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_simple_addition() {
        let input = "1 + 2";
        let expected = AstNode::BinaryOp(
            Box::new(AstNode::Number(1.0)),
            Operator::Add,
            Box::new(AstNode::Number(2.0)),
        );
        assert_eq!(parse_expression(input), expected);
    }

    #[test]
    fn test_addition_and_multiplication() {
        let input = "1 + 2 * 3";
        let expected = AstNode::BinaryOp(
            Box::new(AstNode::Number(1.0)),
            Operator::Add,
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::Number(2.0)),
                Operator::Multiply,
                Box::new(AstNode::Number(3.0)),
            )),
        );
        assert_eq!(parse_expression(input), expected);
    }

    #[test]
    fn test_parentheses() {
        let input = "(1 + 2) * 3";
        let expected = AstNode::BinaryOp(
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::Number(1.0)),
                Operator::Add,
                Box::new(AstNode::Number(2.0)),
            )),
            Operator::Multiply,
            Box::new(AstNode::Number(3.0)),
        );
        assert_eq!(parse_expression(input), expected);
    }

    #[test]
    fn test_division_and_subtraction() {
        let input = "10 / 2 - 3";
        let expected = AstNode::BinaryOp(
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::Number(10.0)),
                Operator::Divide,
                Box::new(AstNode::Number(2.0)),
            )),
            Operator::Subtract,
            Box::new(AstNode::Number(3.0)),
        );
        assert_eq!(parse_expression(input), expected);
    }

    #[test]
    fn test_implicit_multiplication_between_number_and_parenthesis() {
        let input = "2(3 + 4)";
        let expected = AstNode::BinaryOp(
            Box::new(AstNode::Number(2.0)),
            Operator::Multiply,
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::Number(3.0)),
                Operator::Add,
                Box::new(AstNode::Number(4.0)),
            )),
        );
        assert_eq!(parse_expression(input), expected);
    }

    #[test]
    fn test_implicit_multiplication_between_parentheses() {
        let input = "(2)(3 + 4)";
        let expected = AstNode::BinaryOp(
            Box::new(AstNode::Number(2.0)),
            Operator::Multiply,
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::Number(3.0)),
                Operator::Add,
                Box::new(AstNode::Number(4.0)),
            )),
        );
        assert_eq!(parse_expression(input), expected);
    }

    #[test]
    fn test_nested_parentheses() {
        let input = "((1 + 2) * (3 + 4))";
        let expected = AstNode::BinaryOp(
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::Number(1.0)),
                Operator::Add,
                Box::new(AstNode::Number(2.0)),
            )),
            Operator::Multiply,
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::Number(3.0)),
                Operator::Add,
                Box::new(AstNode::Number(4.0)),
            )),
        );
        assert_eq!(parse_expression(input), expected);
    }

    #[test]
    fn test_complex_expression() {
        let input = "3 + 4 * 2 / (1 - 5)";
        let expected = AstNode::BinaryOp(
            Box::new(AstNode::Number(3.0)),
            Operator::Add,
            Box::new(AstNode::BinaryOp(
                Box::new(AstNode::BinaryOp(
                    Box::new(AstNode::Number(4.0)),
                    Operator::Multiply,
                    Box::new(AstNode::Number(2.0)),
                )),
                Operator::Divide,
                Box::new(AstNode::BinaryOp(
                    Box::new(AstNode::Number(1.0)),
                    Operator::Subtract,
                    Box::new(AstNode::Number(5.0)),
                )),
            )),
        );
        assert_eq!(parse_expression(input), expected);
    }
}
