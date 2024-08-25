use crate::Span;

use super::PklStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct Extends<'a> {
    pub name: &'a str,
    pub span: Span,
}

impl<'a> Extends<'a> {
    pub fn not_allowed_here_err(&self) -> String {
        String::from("Keyword `extends` is not allowed here. (If you must use this name as identifier, enclose it in backticks.)")
    }
}

impl<'a> From<Extends<'a>> for PklStatement<'a> {
    fn from(value: Extends<'a>) -> Self {
        PklStatement::ExtendsClause(value)
    }
}
