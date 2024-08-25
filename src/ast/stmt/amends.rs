use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Amends<'a> {
    pub name: &'a str,
    pub span: Span,
}

impl<'a> Amends<'a> {
    pub fn not_allowed_here_err(&self) -> String {
        String::from("Keyword `amends` is not allowed here. (If you must use this name as identifier, enclose it in backticks.)")
    }
}
