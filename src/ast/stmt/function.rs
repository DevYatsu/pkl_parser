use super::ModifiersList;
use crate::{
    ast::{expr::Expr, types::Type},
    Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub modifiers: ModifiersList,
    pub name: (&'a str, Span),
    pub params: Vec<(&'a str, Span)>,
    pub return_type: Option<Type>,
    pub value: Expr,
}
