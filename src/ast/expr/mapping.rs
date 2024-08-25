use crate::Span;

use super::{
    object::{ForGenerator, SpreadObject, WhenGenerator},
    Expr, Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    fields: Vec<MappingField>,
    /// The name of the amended object
    amends: Option<(String, Span)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MappingField {
    DefaultField(Expr),
    When(WhenGenerator),
    For(ForGenerator),
    Spread(SpreadObject),
    Property {
        key: (String, Span),
        value: Expr,
    },
    LocalProp {
        name: (String, Span),
        type_: Option<Type>,
        value: Expr,
    },
    LateBoundField {
        key: Expr,
        index: Expr,
        value: Expr,
    },
}
