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
