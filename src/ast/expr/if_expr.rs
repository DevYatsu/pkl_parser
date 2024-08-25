use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    condition: Box<Expr>,
    if_block: Box<Expr>,
    else_block: Box<Expr>,
}
