use crate::{parse_as_pairs, PklResult, Rule, Span};
use expr::Expr;
use pest::iterators::Pairs;
use stmt::PklStatement;

mod expr;
mod stmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub name: (String, Span),
    pub params: Option<Vec<(Type, Span)>>,
    pub restraints: Option<Box<Expr>>,
}

pub fn parse_as_ast<'a>(src: &str) -> PklResult<Vec<PklStatement<'a>>> {
    let mut pairs = parse_as_pairs(src)?;
    let mut stmts = vec![];
    Ok(stmts)
}
