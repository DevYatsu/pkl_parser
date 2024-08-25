use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub fragments: Vec<StrFragment>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrFragment {
    Literal(String),
    Escaped(char),
    Expr(Expr),
}
