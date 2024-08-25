use super::{Expr, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct FnExpr {
    params: Vec<(String, Option<Type>)>,
    value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnCall {
    name: String,
    args: Vec<Expr>,
}
