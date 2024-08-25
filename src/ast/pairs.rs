use super::{
    annotation::Annotation,
    doc_comment::DocComment,
    expr::Expr,
    stmt::{amends::Amends, import::Import, module::Module},
};
use crate::{
    ast::stmt::{extends::Extends, ModifiersList},
    PklResult, Rule, Span,
};
use pest::iterators::Pair;

trait AsRng {
    fn as_rng(&self) -> Span;
}

impl<'a> AsRng for pest::Span<'a> {
    fn as_rng(&self) -> Span {
        self.start()..self.end()
    }
}

fn pkl_str_content(s: &str) -> &str {
    &s[1..s.len() - 1]
}

fn ident(element: Pair<Rule>) -> PklResult<&str> {
    for x in element.into_inner() {
        match x.as_rule() {
            Rule::ident => return Ok(x.as_str()),
            _ => unreachable!("ident"),
        }
    }

    unreachable!("ident")
}

fn expr(element: Pair<Rule>) -> PklResult<Expr> {
    todo!()
}

pub fn doc_comment(element: Pair<Rule>) -> PklResult<DocComment> {
    let mut comment_text = element.as_str().to_string();
    comment_text = comment_text
        .strip_prefix("///")
        .unwrap()
        .split("\n///")
        .map(|s| if s.trim().len() == 0 { "\n" } else { s.trim() })
        .collect::<Vec<_>>()
        .join(" ");

    Ok(DocComment(comment_text))
}

pub fn annotation(element: Pair<Rule>) -> PklResult<Annotation> {
    for annotation_expr in element.into_inner() {
        match annotation_expr.as_rule() {
            Rule::expr => return Ok(Annotation(Some(expr(annotation_expr)?))),
            _ => unreachable!(),
        }
    }

    Ok(Annotation(None))
}

pub fn amends(element: Pair<Rule>) -> PklResult<Amends> {
    let span = element.as_span();
    for inner_elements in element.into_inner() {
        match inner_elements.as_rule() {
            Rule::src => {
                let span = span.start()..span.end();
                return Ok(Amends {
                    name: pkl_str_content(inner_elements.as_str()),
                    span,
                });
            }
            _ => unreachable!(),
        }
    }

    unreachable!("amends src should be already parsed")
}

pub fn import(element: Pair<Rule>) -> PklResult<Import> {
    let span = element.as_span();
    let span = span.start()..span.end();
    let mut name = "";
    let mut is_globbed = false;
    let mut local_name = None;

    for inner_element in element.into_inner() {
        match inner_element.as_rule() {
            Rule::src => name = pkl_str_content(inner_element.as_str()),
            Rule::import_as => local_name = Some(ident(inner_element)?),
            Rule::globbed_import => is_globbed = true,

            // x => {
            //     println!("unreachable rule {x:?}")
            // }
            _ => unreachable!(),
        }
    }

    Ok(Import {
        name,
        local_name,
        span,
        is_globbed,
    })
}

pub fn module(element: Pair<Rule>) -> PklResult<Module> {
    let mut modifiers = ModifiersList::new();
    for inner_elements in element.into_inner() {
        match inner_elements.as_rule() {
            Rule::PREFIX_KEYWORD => {
                modifiers.push(inner_elements.as_str());
            }
            Rule::ident_with_opt_dots => {
                return Ok(Module {
                    modifiers,
                    full_name: (inner_elements.as_str(), inner_elements.as_span().as_rng()),
                });
            }
            _ => unreachable!(),
        }
    }

    unreachable!("module src should be already parsed")
}

pub fn extends(element: Pair<Rule>) -> PklResult<Extends> {
    let span = element.as_span();
    for inner_elements in element.into_inner() {
        match inner_elements.as_rule() {
            Rule::src => {
                let span = span.start()..span.end();
                return Ok(Extends {
                    name: pkl_str_content(inner_elements.as_str()),
                    span,
                });
            }
            _ => unreachable!(),
        }
    }

    unreachable!("extends src should be already parsed")
}
