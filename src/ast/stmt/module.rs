use crate::Span;

use super::ModifiersList;

#[derive(Debug, Clone, PartialEq)]
pub struct Module<'a> {
    pub modifiers: ModifiersList,
    pub full_name: (&'a str, Span),
}

impl<'a> Module<'a> {
    pub fn last_name_component(&self) -> &str {
        &self.full_name.0.split('.').last().unwrap()
    }

    pub fn not_allowed_here_err(&self) -> String {
        String::from("Keyword `module` is not allowed here. (If you must use this name as identifier, enclose it in backticks.)")
    }
}
