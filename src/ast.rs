use crate::{parse_as_pairs, PklResult, Rule};
use annotation::Annotation;
use doc_comment::DocComment;
use stmt::PklStatement;

pub mod annotation;
pub mod doc_comment;
pub mod expr;
mod pairs;
pub mod stmt;
pub mod types;

pub fn parse_as_ast<'a>(src: &'a str) -> PklResult<Vec<PklStatement<'a>>> {
    let mut pairs = parse_as_pairs(src)?;
    let mut stmts = vec![];

    let file = pairs.next().unwrap().into_inner();

    let mut doc_comment: Option<DocComment> = None;
    let mut annotations: Option<Vec<Annotation>> = None;

    for element in file {
        match element.as_rule() {
            // then take care of each stmt separately
            Rule::stmt => {
                let pair = element.into_inner().next().unwrap(/* safe */);
                let rule = pair.as_rule();

                let mut stmt = match rule {
                    Rule::property => {
                        todo!()
                        // let prop = match_property(&mut table, pair);c
                    }
                    Rule::amends => pairs::amends(pair)?.into(),
                    Rule::import => pairs::import(pair)?.into(),
                    Rule::module => pairs::module(pair)?.into(),
                    Rule::extends => pairs::extends(pair)?.into(),
                    Rule::typealias => pairs::typealias(pair)?.into(),
                    _ => unreachable!(),
                };

                if annotations.is_some() {
                    stmt = PklStatement::WithAnnotation(annotations.take().unwrap(), Box::new(stmt))
                }
                if doc_comment.is_some() {
                    stmt = PklStatement::WithDocComment(doc_comment.take().unwrap(), Box::new(stmt))
                }

                stmts.push(stmt);
            }

            // comments done
            Rule::COMMENT => {
                let pair = element.to_owned().into_inner().next().unwrap();

                match pair.as_rule() {
                    Rule::doc_comment => {
                        let comment = pairs::doc_comment(pair)?;
                        doc_comment = Some(comment);
                    }
                    Rule::annotation => {
                        let annotation = pairs::annotation(pair)?;

                        annotations = if annotations.is_some() {
                            let mut x = annotations.take().unwrap();
                            x.push(annotation);
                            Some(x)
                        } else {
                            Some(vec![annotation])
                        };
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
