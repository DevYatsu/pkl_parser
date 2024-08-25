use super::{Expr, Type};

/// A let expression is Pkl’s version of an (immutable) local variable. Its syntax is:
///
/// ```let (<name> = <value>) <expr>```
#[derive(Debug, Clone, PartialEq)]
pub struct LetExpr {
    name: Option<String>,
    _type: Option<Type>,
    value: Box<Expr>,
    expr: Box<Expr>,
}
