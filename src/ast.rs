use crate::{parse_as_pairs, PklResult, Rule, Span};
use expr::Expr;
use stmt::PklStatement;

mod annotation;
mod doc_comment;
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

    let file = pairs.next().unwrap().into_inner();

    let mut doc_comment: Option<String> = None;
    // let mut annotations: Option<Vec<_>> = None;

    for element in file {
        match element.as_rule() {
            // then take care of each stmt separately
            Rule::stmt => {
                let pair = element.into_inner().next().unwrap(/* safe */);
                let rule = pair.as_rule();
                match rule {
                    Rule::property => {
                        let prop = match_property(&mut table, pair);
                    }
                    _ => unreachable!(),
                }
            }
            // then take care of each comment/annotation separately
            Rule::COMMENT => {
                let pair = element.to_owned().into_inner().next().unwrap();
                match pair.as_rule() {
                    Rule::doc_comment => {
                        let mut comment_text = element.as_str().to_string();
                        comment_text = comment_text
                            .strip_prefix("///")
                            .unwrap()
                            .split("\n///")
                            .map(|s| if s.trim().len() == 0 { "\n" } else { s })
                            .collect::<Vec<_>>()
                            .join(" ");

                        doc_comment = Some(comment_text);
                    }
                    Rule::annotation => {}
                    Rule::line_comment => {}
                    Rule::multiline_comment => {}
                    _ => unreachable!(),
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(stmts)
}
