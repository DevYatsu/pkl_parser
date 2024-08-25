use super::Expr;
use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    lhs: Box<Expr>,
    op: (Operator, Span),
    rhs: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Pow,
    Mul,
    Div,
    DivRound,
    Modulo,
    And,
    Or,
    Pipe,
    CompEqual,
    CompNotEqual,
    CompGreaterEqual,
    CompGreater,
    CompLessEqual,
    CompLess,
    NullCoalescing,

    Is,
    As,
}
