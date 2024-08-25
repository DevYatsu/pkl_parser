use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Import<'a> {
    pub name: &'a str,
    pub local_name: Option<&'a str>,
    pub span: Span,
    pub is_globbed: bool,
}

impl<'a> Import<'a> {
    pub fn not_allowed_here_err(&self) -> String {
        String::from("Keyword `import` is not allowed here. (If you must use this name as identifier, enclose it in backticks.)")
    }
}
