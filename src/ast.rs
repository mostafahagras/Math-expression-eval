#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Number(f64),
    BinaryOp(Box<AstNode>, Operator, Box<AstNode>),
    UnaryOp(UnaryOp, Box<AstNode>),
    Group(Box<AstNode>),
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
            AstNode::Group(grp) => grp.eval(),
            AstNode::BinaryOp(lhs, op, rhs) => op.eval(lhs, rhs),
            AstNode::Number(number) => *number,
            AstNode::UnaryOp(unary_op, number) => unary_op.eval(number),
        }
    }
}
