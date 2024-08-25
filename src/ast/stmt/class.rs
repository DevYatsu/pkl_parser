use super::{function::Function, ModifiersList};
use crate::{
    ast::{expr::Expr, types::Type},
    Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration<'a> {
    pub modifiers: ModifiersList,
    pub name: (&'a str, Span),
    pub extends: Option<(&'a str, Span)>,
    pub fields: Vec<ClassField<'a>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassField<'a> {
    Property {
        name: (&'a str, Span),
        modifiers: ModifiersList,
        _type: Option<Type>,
        value: Option<Expr>,
    },
    Method(Function<'a>),
}
