use crate::Span;

use self::{
    binary::BinaryExpr,
    boxed::{LogicalNot, NonNull, Parenthesized},
    fn_expr::{FnCall, FnExpr},
    if_expr::IfExpr,
    indexing::Indexing,
    let_expr::LetExpr,
    listing::Listing,
    mapping::Mapping,
    object::Object,
    str::Str,
};

use super::Type;

pub mod binary;
pub mod boxed;
pub mod fn_expr;
pub mod if_expr;
pub mod indexing;
pub mod let_expr;
pub mod listing;
pub mod mapping;
pub mod object;
pub mod str;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Null(Span),
    String(Str, Span),
    Int(i64, Span),
    Float(f64, Span),
    Bool(bool, Span),

    Ident(String, Span),
    Indexing(Indexing, Span),

    Object(Object),
    Listing(Listing),
    Mapping(Mapping),

    ClassInstance((String, Span), Object),

    If(IfExpr),
    Let(LetExpr),
    Fn(FnExpr),
    FnCall(FnCall),

    Parenthesized(Parenthesized),
    NonNull(NonNull),
    LogicalNot(LogicalNot),

    Binary(BinaryExpr),

    // for is and as operators
    Type(Type),
}
