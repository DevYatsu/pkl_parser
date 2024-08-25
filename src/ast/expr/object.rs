use crate::Span;

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    fields: Vec<ObjectField>,
    /// The name of the amended object
    amends: Option<(String, Span)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectField {
    Property { name: (String, Span), value: Expr },
    When(WhenGenerator),
    For(ForGenerator),
    Spread(SpreadObject),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhenGenerator {
    condition: Box<Expr>,
    true_block: Box<Expr>,
    false_block: Option<Box<Expr>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ForGenerator {
    indexed: Box<Expr>,
    indexors: Vec<(String, Span)>,
    block: Box<Expr>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SpreadObject {
    is_nullable: bool,
    object: Box<Expr>,
}
