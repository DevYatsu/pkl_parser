use super::Expr;

pub trait IntoInner {
    fn into_inner(self) -> Expr;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parenthesized(Box<Expr>);

#[derive(Debug, Clone, PartialEq)]
pub struct NonNull(Box<Expr>);

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalNot(Box<Expr>);

impl IntoInner for Parenthesized {
    fn into_inner(self) -> Expr {
        *self.0
    }
}

impl IntoInner for NonNull {
    fn into_inner(self) -> Expr {
        *self.0
    }
}

impl IntoInner for LogicalNot {
    fn into_inner(self) -> Expr {
        *self.0
    }
}
