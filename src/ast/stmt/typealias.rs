use super::{ModifiersList, PklStatement};
use crate::{ast::Type, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias<'a> {
    pub modifiers: ModifiersList,
    pub name: (&'a str, Span),
    pub attributes: Vec<(&'a str, Span)>,
    pub refering_type: Type,
}

impl<'a> TypeAlias<'a> {
    pub fn not_allowed_here_err(&self) -> String {
        String::from("Keyword `typealias` is not allowed here. (If you must use this name as identifier, enclose it in backticks.)")
    }
    pub fn modifier_not_applicable_err(&self, modifier: &str) -> String {
        format!("Modifier `{modifier}` is not applicable to type aliases.")
    }
}

impl<'a> From<TypeAlias<'a>> for PklStatement<'a> {
    fn from(value: TypeAlias<'a>) -> Self {
        PklStatement::TypeAlias(value)
    }
}
