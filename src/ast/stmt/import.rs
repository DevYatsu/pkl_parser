use crate::Span;

use super::PklStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct Import<'a> {
    pub name: (&'a str, Span),
    pub local_name: Option<(&'a str, Span)>,
    pub span: Span,
    pub is_globbed: bool,
}

impl<'a> Import<'a> {
    pub fn not_allowed_here_err(&self) -> String {
        String::from("Keyword `import` is not allowed here. (If you must use this name as identifier, enclose it in backticks.)")
    }
}

impl<'a> From<Import<'a>> for PklStatement<'a> {
    fn from(value: Import<'a>) -> Self {
        PklStatement::Import(value)
    }
}
