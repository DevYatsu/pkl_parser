use crate::Span;

use super::{
    object::{ForGenerator, SpreadObject, WhenGenerator},
    Expr, Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Listing {
    fields: Vec<ListingField>,
    /// The name of the amended object
    amends: Option<(String, Span)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListingField {
    DefaultField(Expr),
    When(WhenGenerator),
    For(ForGenerator),
    Spread(SpreadObject),
    Element(Expr),
    LocalProp {
        name: (String, Span),
        type_: Option<Type>,
        value: Expr,
    },
    AmendField {
        index: Expr,
        value: Expr,
    },
    LateBoundField {
        index: Expr,
        value: Expr,
    },
}
