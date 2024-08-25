use super::ModifiersList;
use crate::{
    ast::{expr::Expr, types::Type},
    Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Property<'a> {
    pub modifiers: ModifiersList,
    pub name: (&'a str, Span),
    pub _type: Option<Type>,
    pub value: Option<Expr>,
}
