use crate::{parse_as_pairs, PklResult, Rule, Span};
use expr::Expr;
use stmt::PklStatement;

mod annotation;
mod doc_comment;
mod expr;
mod stmt;

/// Represents a type in the `Pkl` language.
///
/// # Overview
///
/// The `Type` struct encapsulates a type definition within the `Pkl` language.
/// A type typically has a name, optional parameters, and optional constraints
/// or restraints associated with it. This struct is designed to hold all
/// relevant information needed to represent a type in the `Pkl` abstract
/// syntax tree (AST).
///
/// # Fields
///
/// * `name: (String, Span)` - This tuple contains the name of the type as a `String`
/// along with its `Span`, which represents the location of the type name in the source code.
///
/// * `params: Option<Vec<(Type, Span)>>` - An optional vector of type parameters,
/// each represented by a tuple containing a `Type` and its associated `Span`.
/// If the type has no parameters, this will be `None`.
///
/// * `restraints: Option<Box<Expr>>` - An optional expression that imposes
/// constraints or restraints on the type. This could be used to enforce certain
/// conditions or requirements on the type. If there are no restraints, this will be `None`.
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
