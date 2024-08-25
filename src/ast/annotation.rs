use super::expr::Expr;

/// A `Pkl` annotation, which may contain an expression.
///
/// # Struct
///
/// This struct represents an annotation in the `Pkl` format. An annotation is
/// essentially optional expressions (`Expr`) that can be attached to various statements,
/// in order to add some context to it.
///
/// # Fields
///
/// * `Option<Expr>` - This field holds the expression associated with the annotation.
/// If the annotation does not contain an expression, this will be `None`.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation(Option<Expr>);
