use crate::{parse_as_pairs, PklResult, Rule, Span};
use annotation::Annotation;
use doc_comment::DocComment;
use expr::Expr;
use stmt::PklStatement;

pub mod annotation;
pub mod doc_comment;
pub mod expr;
mod pairs;
pub mod stmt;

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

pub fn parse_as_ast<'a>(src: &'a str) -> PklResult<Vec<PklStatement<'a>>> {
    let mut pairs = parse_as_pairs(src)?;
    let mut stmts = vec![];

    let file = pairs.next().unwrap().into_inner();

    let mut doc_comment: Option<DocComment> = None;
    let mut annotations: Vec<Annotation> = Vec::new();

    for element in file {
        match element.as_rule() {
            // then take care of each stmt separately
            Rule::stmt => {
                let pair = element.into_inner().next().unwrap(/* safe */);
                let rule = pair.as_rule();

                let stmt = match rule {
                    Rule::property => {
                        todo!()
                        // let prop = match_property(&mut table, pair);
                    }
                    Rule::amends => pairs::amends(pair)?.into(),
                    Rule::import => pairs::import(pair)?.into(),
                    Rule::module => pairs::module(pair)?.into(),
                    Rule::extends => pairs::extends(pair)?.into(),
                    _ => unreachable!(),
                };
                stmts.push(stmt);
            }

            // comments done
            Rule::COMMENT => {
                let pair = element.to_owned().into_inner().next().unwrap();
                match pair.as_rule() {
                    Rule::doc_comment => {
                        let comment = pairs::doc_comment(element)?;
                        doc_comment = Some(comment);
                    }
                    Rule::annotation => {
                        let annotation = pairs::annotation(element)?;
                        annotations.push(annotation);
                    }
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
